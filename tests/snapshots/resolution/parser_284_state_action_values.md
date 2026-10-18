# META
~~~ini
description=AST 284 state and perform values retain resolved expression operands and missing references
libraries=none
require_complete_publication=true
~~~
# SOURCE
~~~sysml
package StateValues {
    action def A;
    action existing : A;
    state def S {
        entry action initialize : A = existing;
        do action periodic : A default = existing;
        exit action finalize : A = missing;
    }
    action def Host {
        perform action repeated : A = existing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_state_action_values.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 35) (end 6 42))
      )
    )
  )
)
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind feature_typing) (source "StateValues::S::initialize") (target "StateValues::A") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "StateValues::Host::repeated") (target "StateValues::A") (provenance authored) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_state_action_values.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 35) (end 6 42))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:6093fec5c65655b3e42b7a2b3d13def084e924345b66cf99f4bec909d24d48c4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::Host"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::Host::repeated"))) (kind perform-action) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "existing")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::finalize"))) (kind exit-action-binding) (membership (kind feature) (visibility default) (role exit)) (feature-value (kind bind) (value (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind exit-action-binding) (name "finalize")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind exit-action-binding) (name "finalize")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default) (role exit)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind exit-action-binding) (name "finalize")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind exit-action-binding) (name "finalize")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "missing")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind exit-action-binding) (name "finalize")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::initialize"))) (kind entry-action-binding) (membership (kind feature) (visibility default) (role entry)) (feature-value (kind bind) (value (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default) (role entry)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "existing")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::periodic"))) (kind do-action-binding) (membership (kind feature) (visibility default) (role do)) (feature-value (kind bind) (value (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (default true)) (authored (membership (kind feature) (visibility default) (role do)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "existing")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::Host::repeated"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "existing")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::finalize"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind exit-action-binding) (name "finalize")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "missing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::initialize"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "existing")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::periodic"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "existing")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::Host::repeated"))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::Host::repeated"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::finalize"))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::finalize"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::initialize"))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::initialize"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::periodic"))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::periodic"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing"))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::Host::repeated"))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::Host"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::Host"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::finalize"))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind exit-action-binding) (name "finalize")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind exit-action-binding) (name "finalize")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind exit-action-binding) (name "finalize")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::initialize"))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::periodic"))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind exit-action-binding) (name "finalize")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")))
      (subtype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::Host::repeated")) (scopes any))
      (subtype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::finalize")) (scopes any))
      (subtype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::initialize")) (scopes any))
      (subtype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::periodic")) (scopes any))
      (subtype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::Host::repeated")))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::Host")))
      (type (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::Host")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (source inherited) (from (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing"))))
      (supertype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (scopes any))
      (supertype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::finalize")))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S")))
      (type (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind exit-action-binding) (name "finalize")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind exit-action-binding) (name "finalize")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind exit-action-binding) (name "finalize")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::initialize")))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S")))
      (type (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (source inherited) (from (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing"))))
      (supertype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (scopes any))
      (supertype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::periodic")))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S")))
      (type (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (source inherited) (from (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing"))))
      (supertype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (scopes any))
      (supertype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing")))
      (type (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")) (scopes any))
      (subtype (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "existing" (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing")))))
  (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind exit-action-binding) (name "finalize")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "missing" (target unresolved)))
  (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "existing" (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing")))))
  (declaration (id (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "existing" (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/parser_284_state_action_values.md") (range (start 9 34) (end 9 35)) (probe (position 9 34))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::Host::repeated"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_values.md") (range (start 9 38) (end 9 46)) (probe (position 9 38))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind action-def) (name "Host")) (named (kind perform-action) (name "repeated")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "existing")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing")))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_values.md") (range (start 6 31) (end 6 32)) (probe (position 6 31))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::finalize"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_values.md") (range (start 6 35) (end 6 42)) (probe (position 6 35))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind exit-action-binding) (name "finalize")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "missing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_values.md") (range (start 4 34) (end 4 35)) (probe (position 4 34))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::initialize"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_values.md") (range (start 4 38) (end 4 46)) (probe (position 4 38))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind entry-action-binding) (name "initialize")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "existing")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing")))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_values.md") (range (start 5 29) (end 5 30)) (probe (position 5 29))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::S::periodic"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_values.md") (range (start 5 41) (end 5 49)) (probe (position 5 41))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (path (named (kind package) (name "StateValues")) (named (kind state-def) (name "S")) (named (kind do-action-binding) (name "periodic")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "existing")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing")))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_values.md") (range (start 2 22) (end 2 23)) (probe (position 2 22))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::existing"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_values.md") (qualified-name "StateValues::A")))))
    )
  )
)
~~~
