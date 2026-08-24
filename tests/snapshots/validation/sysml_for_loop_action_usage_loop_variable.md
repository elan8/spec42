# META
~~~ini
description=SysML 8.3.17.9 validateForLoopActionUsageLoopVariable requires the first ownedFeature of a ForLoopActionUsage to be a ReferenceUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.9 validateForLoopActionUsageLoopVariable
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.9:validateForLoopActionUsageLoopVariable
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the loop variable below is the first owned feature of the for loop and is a
// reference usage, which is what the for syntax produces.
//
// The violating side has no textual counterpart: SysML for syntax always authors the loop
// variable first and always as a reference usage.
// Note: the pinned parser now models the loop variable as a ForVariableDeclaration, so the
// accepted side is observable in SMG.
package Actions {
    part def Component;
    action def Act {
        ref part components : Component[0..*];
        for c : Component in components { action step; }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:ae2fd71881a0ebf2bc55bf49d074c5bb96a0943f59f5626d4fe47d15b07db14c") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0))))) (kind for-loop) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "components")))))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "c"))))) (kind for-loop-variable) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "step"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act::components"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "components")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act::components")))))
    (reference (id (source (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act::components"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Component")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act::components"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act::components"))) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act::components"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "c"))))) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "step"))))) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act::components"))) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "c")))))
      (featured-by (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "step")))))
      (featured-by (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act::components")))
      (featured-by (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act")))
      (type (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Component")))
      (subtype (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act::components")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (range (start 11 29) (end 11 39)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind for-loop) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "components")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act::components")))))
    )
  )
  (query (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (range (start 10 30) (end 10 39)) (probe (position 10 30))
    (reference (id (source (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act::components"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Component")))))
    )
  )
)
~~~
