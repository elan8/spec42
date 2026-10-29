# META
~~~ini
description=A `then assign` is a succession from the preceding member to the assignment node, also inside a loop body; a plain `assign` has no incoming succession; SysML 2.0 8.2.2.17.1, 8.2.2.17.5
type=generate
libraries=standard
plugin=native:diagram
viewKind=action-flow-view
viewDocument=model.sysml
viewQualifiedName=Counting::countFlow
require_no_diagnostics=true
require_complete_publication=true
~~~
# SOURCE
## model.sysml
~~~sysml
package Counting {
    private import StandardViewDefinitions::*;
    private import ScalarValues::*;
    action def Sample;
    action def Count {
        attribute count : Integer := 0;
        first start;
        then assign count := 0;
        then while count < 3 {
            action sample : Sample;
            then assign count := count + 1;
        }
        then done;
        assign count := 5;
    }
    view countFlow : ActionFlowView { expose Count; }
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:20e410ade96c94f200ad731ccb6d064886dfec77ec3d51a79e7dc40b525bacca") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Counting"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "start")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (assignTarget (reference "count")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0))))) (kind while) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "count")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind then-continuation) (ordinal 0))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "done")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (assignTarget (reference "count")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "count")) (assignTarget (reference "count")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "sample"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sample")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind assign) (value (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Sample"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::countFlow"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ActionFlowView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind view) (name "countFlow")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Count")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0))
      (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0))
      (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0))
      (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0))
      (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "sample"))))) (kind featureTyping) (ordinal 0))
      (authored-target "Sample")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Sample")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::countFlow"))) (kind featureTyping) (ordinal 0))
      (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind view) (name "countFlow")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0)))
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "sample"))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Sample"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "sample"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::countFlow"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::countFlow"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind view) (name "countFlow")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind view) (name "countFlow")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::whileLoops"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::whileLoopActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments::target"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments::target"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions::target"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions::target"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments::target"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions::target"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt::accessedFeature"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt::accessedFeature"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt::accessedFeature"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "sample"))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "sample"))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "sample"))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Sample"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::countFlow"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind view) (name "countFlow")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::countFlow"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0))))) (state literal) (value (kind integer) (integer 0)))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0))))) (state evaluated) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1))))) (state literal) (value (kind integer) (integer 5)))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (state evaluated) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 0)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::LoopAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::loops"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::LoopAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::loopActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::WhileLoopAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::whileLoops"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::WhileLoopAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::whileLoopActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::loops")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::whileLoops")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::LoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::WhileLoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::loopActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::whileLoopActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::LoopPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind then-continuation) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt::accessedFeature"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")) (scopes any feature))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt::accessedFeature"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")) (scopes any feature))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt::accessedFeature"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")) (scopes any feature))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "sample")))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)))))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Sample")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Sample")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Sample")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count")))
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
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Sample")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "sample")))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::countFlow")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind view) (name "countFlow")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::countFlow")))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 0))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0))))) (outcome resolved) (operator "<" (feature-reference "count" (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")))) (literal (value (kind integer) (integer 3)))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1))))) (outcome resolved) (literal (value (kind integer) (integer 5))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (outcome resolved) (operator "+" (feature-reference "count" (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")))) (literal (value (kind integer) (integer 1)))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 0))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 2 19) (end 2 34)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 14) (end 6 19)) (probe (position 6 14))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 19) (end 8 24)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 13) (end 12 17)) (probe (position 12 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0) (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 20) (end 7 25)) (probe (position 7 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0) (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 15) (end 13 20)) (probe (position 13 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0) (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 33) (end 10 38)) (probe (position 10 33))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 24) (end 10 29)) (probe (position 10 24))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0) (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 28) (end 9 34)) (probe (position 9 28))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind action-def) (name "Count")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "sample"))))) (kind featureTyping) (ordinal 0) (authored-target "Sample")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Sample")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 26) (end 5 33)) (probe (position 5 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count::count"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 21) (end 15 35)) (probe (position 15 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::countFlow"))) (kind featureTyping) (ordinal 0) (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 45) (end 15 50)) (probe (position 15 45))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Counting")) (named (kind view) (name "countFlow")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Counting::Count")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:e2a610b5d161a77a1e5af38dd56704b92f85a46abcc27f49209412ac4773c955",
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
        4,
        15,
        4,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        18,
        5,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        26,
        5,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        8,
        6,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        14,
        6,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        8,
        7,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        20,
        7,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        13,
        11,
        9
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        19,
        8,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        19,
        9,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        28,
        9,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        12,
        10,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        24,
        10,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        33,
        10,
        38
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        8,
        12,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        13,
        12,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        8,
        13,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        15,
        13,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        9,
        15,
        18
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
      "qualifiedName": "Counting::Count"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Counting::Count::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Counting::Count::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Counting::Count::::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Counting::Count::::::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Counting::Count::::sample"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Counting::Count::count"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Counting::Sample"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Counting::countFlow"
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
      "qualifiedName": "ScalarValues::Integer"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AssignmentActionUsage",
      "ownerQualifiedName": "Counting::Count",
      "source": 5,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AssignmentActionUsage",
      "ownerQualifiedName": "Counting::Count",
      "source": 16,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Counting::Count",
      "source": 3,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Counting::Count",
      "source": 14,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "WhileLoopActionUsage",
      "ownerQualifiedName": "Counting::Count",
      "source": 7,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AssignmentActionUsage",
      "ownerQualifiedName": "Counting::Count::",
      "source": 11,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Counting::Count::",
      "source": 6,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Counting::Count::",
      "source": 17,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Counting::Count::::",
      "source": 6,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Counting::Count::::",
      "source": 17,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Counting::Count::::",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Counting::Count::::::",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Counting::Count::::::",
      "source": 6,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Counting::Count::::::",
      "source": 17,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Counting::Count::::::::",
      "source": 12,
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
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 13,
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
      "ordinal": 18,
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
      "ordinal": 22,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "assignTarget",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "assignTarget",
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
      "ordinal": 7,
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
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "expressionOperand",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 50,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 56,
      "relationshipKind": "thenTarget",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "assignTarget",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 11,
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
      "ordinal": 6,
      "relationshipKind": "expressionOperand",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "succession",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "typing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 52,
      "relationshipKind": "typing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 59,
      "relationshipKind": "redefinition",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 58,
      "relationshipKind": "typing",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 64,
      "relationshipKind": "redefinition",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "succession",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 63,
      "relationshipKind": "typing",
      "source": 11
    }
  ],
  "selectedView": {
    "reference": 8,
    "kind": "action-flow-view",
    "name": "countFlow",
    "source": 18
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 3,
        "provenance": "authored",
        "reference": 30,
        "source": 0,
        "target": 3
      },
      {
        "kind": "succession",
        "navigation": 7,
        "origin": 3,
        "provenance": "authored",
        "reference": 45,
        "source": 9,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 5,
        "provenance": "authored",
        "reference": 40,
        "source": 3,
        "target": 5
      },
      {
        "kind": "succession",
        "navigation": 11,
        "origin": 5,
        "provenance": "authored",
        "reference": 58,
        "source": 4,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 6,
        "provenance": "authored",
        "reference": 50,
        "source": 5,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 7,
        "provenance": "authored",
        "reference": 54,
        "source": 6,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 8,
        "provenance": "authored",
        "reference": 57,
        "source": 7,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 4,
        "provenance": "authored",
        "reference": 41,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 9,
        "provenance": "authored",
        "reference": 31,
        "source": 0,
        "target": 9
      },
      {
        "kind": "succession",
        "navigation": 5,
        "origin": 9,
        "provenance": "authored",
        "reference": 64,
        "source": 19,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 10,
        "provenance": "authored",
        "reference": 42,
        "source": 9,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 11,
        "provenance": "authored",
        "reference": 51,
        "source": 10,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 12,
        "provenance": "authored",
        "reference": 55,
        "source": 11,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 13,
        "provenance": "authored",
        "reference": 32,
        "source": 0,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 17,
        "origin": 14,
        "provenance": "authored",
        "reference": 43,
        "source": 13,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 17,
        "origin": 15,
        "provenance": "authored",
        "reference": 52,
        "source": 14,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 17,
        "origin": 16,
        "provenance": "authored",
        "reference": 56,
        "source": 15,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 1,
        "provenance": "authored",
        "reference": 33,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 17,
        "provenance": "authored",
        "reference": 34,
        "source": 0,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 2,
        "provenance": "authored",
        "reference": 35,
        "source": 0,
        "target": 2
      },
      {
        "kind": "succession",
        "navigation": 14,
        "origin": 2,
        "provenance": "authored",
        "reference": 46,
        "source": 3,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 22,
        "origin": 18,
        "provenance": "implied",
        "reference": 36,
        "source": 0,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 19,
        "provenance": "implied",
        "reference": 37,
        "source": 0,
        "target": 19
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "action-flow-view",
    "metadata": {
      "actions": [
        0,
        3,
        4,
        5,
        9,
        13,
        18,
        19
      ],
      "controlNodes": []
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              17
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              3,
              9,
              13
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              18,
              19
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionDefinition",
        "multiplicity": null,
        "name": "Count",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:b3f13b725b3d772855bd06fed86a7bc61fc9a90842e87a96fbbe124a0c92ad5b",
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
        "presentationKey": "p:34ec7c50444d41e30d58f88937202fba014666c739fc993ab39fd38096947423",
        "reference": 17,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:48f1068e33b7287735e5c9e060e8095aada682e2055db9a96c453d61bfa11bb8",
        "reference": 18,
        "source": 14,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "actions",
            "members": [
              4,
              5
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "WhileLoopActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:adb49681cd35230bc6c216f3c178428f085337a27b520828404760b1b21eab96",
        "reference": 19,
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
        "name": "sample",
        "notationRole": "usage",
        "owner": 3,
        "presentationKey": "p:633e3ea32c5384b7ae60dd596419a2521d73255e627daa3dd88b92fb3a81b6ab",
        "reference": 5,
        "source": 9,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Sample",
              "reference": 7
            }
          ]
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
        "metaclass": "AssignmentActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 3,
        "presentationKey": "p:da9bf736725d9655a02f8a24fd556fe82899d05a959729de9f37f48ea2ccb620",
        "reference": 20,
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
        "direction": "in",
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 5,
        "presentationKey": "p:5678289e1271ee63b60b4ecd2f2288fa04a2eb5817f97cd8179da93628468a1a",
        "reference": 25,
        "source": 12,
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
              8
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
        "owner": 6,
        "presentationKey": "p:45c960f9cac51ba4bbe57bb07f948039ae5933a89ac6db46a8305991acf1b204",
        "reference": 26,
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
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 7,
        "presentationKey": "p:107cb9f21565d2a25eb2c357c7dab9c5f43be3e2d31189136b44e1214febef29",
        "reference": 29,
        "source": 12,
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
              10
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "AssignmentActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:72f00094d390b1069e428821ef2953ca8cdcde9aa6883b2988123227cc53c958",
        "reference": 15,
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
        "owner": 9,
        "presentationKey": "p:291d70f90218c4d4374b8adf449c3d98cfd2da094d04718eba2691821947c5fc",
        "reference": 21,
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
              12
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
        "owner": 10,
        "presentationKey": "p:715fa7408bf428ffb2f92443fd156fc77eb68e1573f2c7d0ede856000a6d2670",
        "reference": 23,
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
        "owner": 11,
        "presentationKey": "p:3be09f01cdfb58b7d8307fa63df18afeb164294419852dfd218f23b69dbf21f8",
        "reference": 27,
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
              14
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "AssignmentActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:ea1e94ad8947bf9cd9d333223413c119305e4ed03b983ef910b16a536e984c5b",
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
        "direction": "in",
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 13,
        "presentationKey": "p:99324d0d56b6595ab959e27210da675830d036f16fd1e34c4889fcff5bea5b8e",
        "reference": 22,
        "source": 17,
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
              16
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
        "owner": 14,
        "presentationKey": "p:899e5c5854e9f5291b66e7597b3d873af10784dd3b87d7395ca54d0462fa5e99",
        "reference": 24,
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
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 15,
        "presentationKey": "p:6ef5a671901b74382acd50ba1154ec11ca69e4591395535501b64e2d1a539c4e",
        "reference": 28,
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
        "metaclass": "AttributeUsage",
        "multiplicity": null,
        "name": "count",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:2fff473637e29868aa23c86c58100a2ea50212ebaccd190f16dfb6cee21ac035",
        "reference": 6,
        "source": 1,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Integer",
              "reference": 14
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
        "presentationKey": "p:273c463491921ff1fd542e4635db5d8f6cb06551477fcc53715d01096258b89f",
        "reference": 10,
        "source": 22,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Action",
              "reference": 9
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
        "presentationKey": "p:94af47c7313b1b96df64de0b4f30fbe355363970a4130a329d3b62979acb3097",
        "reference": 11,
        "source": 19,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Action",
              "reference": 9
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "expressionOperand",
        "navigation": 8,
        "provenance": "authored",
        "reference": 44,
        "source": 3,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "assignTarget",
        "navigation": 12,
        "provenance": "authored",
        "reference": 49,
        "source": 5,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "expressionOperand",
        "navigation": 13,
        "provenance": "authored",
        "reference": 53,
        "source": 5,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 10,
        "provenance": "authored",
        "reference": 59,
        "source": 4,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "assignTarget",
        "navigation": 6,
        "provenance": "authored",
        "reference": 38,
        "source": 9,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "assignTarget",
        "navigation": 17,
        "provenance": "authored",
        "reference": 39,
        "source": 13,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 4,
        "provenance": "authored",
        "reference": 47,
        "source": 1,
        "target": {
          "node": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 60,
        "source": 17,
        "target": {
          "reference": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "thenTarget",
        "navigation": 15,
        "provenance": "authored",
        "reference": 48,
        "source": 2,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 23,
        "provenance": "authored",
        "reference": 62,
        "source": 18,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 24,
        "provenance": "authored",
        "reference": 61,
        "source": 18,
        "target": {
          "reference": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 20,
        "provenance": "authored",
        "reference": 65,
        "source": 19,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 21,
        "provenance": "authored",
        "reference": 63,
        "source": 19,
        "target": {
          "reference": 13,
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
