# META
~~~ini
description=Generated IfActionUsage specialization selects the exact else-action library anchor from SysML 8.3.17.10
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.10:checkIfActionUsageSpecialization
coverage_role=secondary
blocked_by=semantic-query-gap-anonymous-library-specialization-forms
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package IfActionElseSpecialization {
    action def Decision {
        action condition;
        if condition {
            action thenAction;
        } else {
            action elseAction;
        }
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "IfActionElseSpecialization::Decision::<anonymous>") (target "Actions::ifThenElseActions") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_if_action_else_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:47da8f6697db9994935fb4fea395a03ad759deca6d9d15d47a1986a074095958") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization::Decision"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (kind if) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "condition")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "elseAction"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "thenAction"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization::Decision::condition"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "condition")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization::Decision::condition")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization::Decision::condition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization::Decision"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ifThenElseActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization::Decision"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "elseAction"))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "elseAction"))))) (target (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "thenAction"))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "thenAction"))))) (target (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization::Decision::condition"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization::Decision::condition"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization::Decision::condition"))) (target (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization::Decision"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization::Decision")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization::Decision")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::IfThenAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::IfThenElseAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ifThenElseActions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::IfPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::IfThenElsePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::IfThenPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "elseAction")))))
      (featured-by (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "thenAction")))))
      (featured-by (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization::Decision::condition")))
      (featured-by (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization::Decision")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (range (start 3 11) (end 3 20)) (probe (position 3 11))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (path (named (kind package) (name "IfActionElseSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "condition")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_if_action_else_specialization.md") (qualified-name "IfActionElseSpecialization::Decision::condition")))))
    )
  )
)
~~~
