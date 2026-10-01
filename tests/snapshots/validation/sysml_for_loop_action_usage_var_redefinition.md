# META
~~~ini
description=SysML 8.3.17.9 checkForLoopActionUsageVarRedefinition requires the loop variable to redefine Actions::ForLoopAction::var
specification=OMG SysML 2.0 (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.9:checkForLoopActionUsageVarRedefinition
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package Redefinition {
    action def Step;
    action def Work {
        in item counts : ScalarValues::Integer[*];
        for count in counts { action step : Step; }
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (redefinition-check (rule_id "sysml-2.0:8.3.17.9:checkForLoopActionUsageVarRedefinition") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:01bc033692e02bc20e962b6f93b327ddab4985341f19330d5662c15d28619bd5") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Step"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0))))) (kind for-loop) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "counts")))))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "count"))))) (kind for-loop-variable) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "step"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Step")))))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work::counts"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Integer")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "counts")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work::counts")))))
    (reference (id (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "step"))))) (kind featureTyping) (ordinal 0))
      (authored-target "Step")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Step")))))
    (reference (id (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work::counts"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work::counts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "step"))))) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Step"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "step"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work::counts"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work::counts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Step"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::forLoopActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "count"))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ForLoopAction::var"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "count"))))) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "step"))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "step"))))) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work::counts"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work::counts"))) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Step")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "step")))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ForLoopAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::forLoopActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::LoopAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::loopActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ForLoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::LoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::forLoopActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::loopActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "count")))))
      (featured-by (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ForLoopAction::seq")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ForLoopAction::var"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ForLoopAction::seq")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ForLoopAction::var")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::LoopAction::iterator")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "step")))))
      (featured-by (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)))))
      (type (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Step")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Step")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Step")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work::counts")))
      (featured-by (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
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
  (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0))))) (outcome resolved) (feature-reference "counts" (target (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work::counts")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (range (start 4 21) (end 4 27)) (probe (position 4 21))
    (reference (id (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "counts")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work::counts")))))
    )
  )
  (query (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (range (start 4 44) (end 4 48)) (probe (position 4 44))
    (reference (id (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "step"))))) (kind featureTyping) (ordinal 0) (authored-target "Step")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Step")))))
    )
  )
  (query (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (range (start 3 25) (end 3 46)) (probe (position 3 25))
    (reference (id (source (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work::counts"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
    )
  )
)
~~~
