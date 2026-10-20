# META
~~~ini
description=Issue 297: retain state action declaration forms
libraries=none
require_complete_publication=true
~~~
# SOURCE
~~~sysml
package P {
    action def A;
    item def S;
    part def Q {
        action ops : A;
        attribute x;
        perform ops : A;
        perform ops :> ops [1];
        state def SD {
            entry ops : A;
            do ops :> ops;
            exit ops [1];
        }
        state def Declared {
            entry action accept S;
            do action sender send S() to self;
            exit action setter assign x := 1;
        }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_state_action_declaration_forms.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 6 8) (end 6 24))
        (related-information
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 4 8) (end 4 23))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 7 23) (end 7 26))
        (related-information
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 4 8) (end 4 23))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 6 8) (end 6 24))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 7 8) (end 7 31))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 9 18) (end 9 21))
        (related-information
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 4 8) (end 4 23))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 6 8) (end 6 24))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 7 8) (end 7 31))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 10 15) (end 10 18))
        (related-information
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 4 8) (end 4 23))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 6 8) (end 6 24))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 7 8) (end 7 31))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 10 22) (end 10 25))
        (related-information
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 4 8) (end 4 23))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 6 8) (end 6 24))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 7 8) (end 7 31))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 11 17) (end 11 20))
        (related-information
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 4 8) (end 4 23))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 6 8) (end 6 24))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 7 8) (end 7 31))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "invocation_instantiated_type_not_behavior")
        (source "semantic")
        (range (start 15 34) (end 15 37))
        (related-information
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 2 4) (end 2 15))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 41) (end 15 45))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_state_action_declaration_forms.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 6 8) (end 6 24))
        (related-information
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 4 8) (end 4 23))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 7 23) (end 7 26))
        (related-information
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 4 8) (end 4 23))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 6 8) (end 6 24))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 7 8) (end 7 31))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 9 18) (end 9 21))
        (related-information
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 4 8) (end 4 23))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 6 8) (end 6 24))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 7 8) (end 7 31))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 10 15) (end 10 18))
        (related-information
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 4 8) (end 4 23))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 6 8) (end 6 24))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 7 8) (end 7 31))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 10 22) (end 10 25))
        (related-information
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 4 8) (end 4 23))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 6 8) (end 6 24))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 7 8) (end 7 31))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 11 17) (end 11 20))
        (related-information
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 4 8) (end 4 23))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 6 8) (end 6 24))
          )
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 7 8) (end 7 31))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "invocation_instantiated_type_not_behavior")
        (source "semantic")
        (range (start 15 34) (end 15 37))
        (related-information
          (related
            (uri "memory://snapshot/parser_284_state_action_declaration_forms.md")
            (range (start 2 4) (end 2 15))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 41) (end 15 45))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:1d4d5a8db5fac92c07d98107b9b90976b22beaddb58a37d442b995fa35580e11"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (effective-identification (name "ops") (short-name absent) (provenance performed-action)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")) (referenceSubsetting (reference "ops")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1))))) (kind perform-action) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (effective-identification (name "ops") (short-name absent) (provenance performed-action)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ops")) (referenceSubsetting (reference "ops")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default) (role entry)) (authored (membership (kind feature) (visibility default) (role entry)) (relationships (expressionOperand (reference "S")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::sender"))) (kind send-action) (membership (kind feature) (visibility default) (role do)) (authored (membership (kind feature) (visibility default) (role do)) (relationships (sendTarget (reference "self")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (invocationCallee (reference "S")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::setter"))) (kind assign) (membership (kind feature) (visibility default) (role exit)) (authored (membership (kind feature) (visibility default) (role exit)) (relationships (assignTarget (reference "x")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::SD"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default) (role entry)) (authored (membership (kind feature) (visibility default) (role entry)) (relationships (featureTyping (reference "A")) (entryActionBinding (reference "ops")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind do-action-binding) (membership (kind feature) (visibility default) (role do)) (authored (membership (kind feature) (visibility default) (role do)) (relationships (subsetting (reference "ops")) (doActionBinding (reference "ops")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exit-action-binding) (membership (kind feature) (visibility default) (role exit)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default) (role exit)) (relationships (exitActionBinding (reference "ops")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::x"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S"))) (kind item-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1))))) (kind subsetting) (ordinal 0))
      (authored-target "ops")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0)))) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)))))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "ops")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "ops")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (anonymous (kind accept-action) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "S")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::sender"))) (kind sendTarget) (ordinal 0))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "S")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::setter"))) (kind assignTarget) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::x")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "ops")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0)))) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)))))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "ops")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0)))) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)))))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0))
      (authored-target "ops")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0)))) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)))))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0))
      (authored-target "ops")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0)))) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)))))))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (anonymous (kind accept-action) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::setter"))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::setter"))) (kind assignTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops"))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::sender"))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::setter"))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::setter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::x"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::SD"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind do-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::SD"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::SD"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::SD"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::SD"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops"))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::x"))) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::setter"))) (state literal) (value (kind integer) (integer 1)))
    (invocation (declaration (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0))))) (callee (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S"))) (supplied 0) (required 0) (start 15 34) (end 15 37))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")))
      (subtype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind entry-action-binding) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q")))
      (type (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")) (source direct))
      (effective-type (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")) (source inherited) (from (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops"))))
      (supertype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")) (scopes any))
      (supertype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q")))
      (effective-type (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")) (source inherited) (from (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops"))))
      (supertype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")) (scopes any))
      (supertype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::sender")))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (type (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S")) (provenance implied))
      (effective-type (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (type (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S")) (provenance implied))
      (effective-type (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::setter")))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::setter")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind parameter) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::x")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::SD")))
      (type (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind do-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::SD")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::SD")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::SD")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::SD")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q")))
      (type (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")) (scopes any))
      (subtype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::x")))
      (featured-by (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q")))
      (subtype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind assign) (name "setter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S")))
      (subtype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::setter"))) (outcome resolved) (literal (value (kind integer) (integer 1))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (range (start 6 22) (end 6 23)) (probe (position 6 22))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (range (start 7 23) (end 7 26)) (probe (position 7 23))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1))))) (kind subsetting) (ordinal 0) (authored-target "ops")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0)))) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)))))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (range (start 6 16) (end 6 19)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "ops")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (range (start 7 16) (end 7 19)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1))))) (kind referenceSubsetting) (ordinal 0) (authored-target "ops")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (range (start 14 32) (end 14 33)) (probe (position 14 32))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (anonymous (kind accept-action) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "S")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S")))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (range (start 15 41) (end 15 45)) (probe (position 15 41))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::sender"))) (kind sendTarget) (ordinal 0) (authored-target "self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (range (start 15 34) (end 15 35)) (probe (position 15 34))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "Declared")) (named (kind send-action) (name "sender")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "S")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::S")))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (range (start 16 38) (end 16 39)) (probe (position 16 38))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::Declared::setter"))) (kind assignTarget) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::x")))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (range (start 9 24) (end 9 25)) (probe (position 9 24))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (range (start 10 22) (end 10 25)) (probe (position 10 22))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "ops")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0)))) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)))))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (range (start 9 18) (end 9 21)) (probe (position 9 18))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "ops")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0)))) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)))))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (range (start 10 15) (end 10 18)) (probe (position 10 15))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0) (authored-target "ops")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0)))) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)))))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (range (start 11 17) (end 11 20)) (probe (position 11 17))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (named (kind state-def) (name "SD")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0) (authored-target "ops")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops")) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 0)))) (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (path (named (kind package) (name "P")) (named (kind part-def) (name "Q")) (anonymous (kind perform-action) (ordinal 1)))))))
    )
  )
  (query (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (range (start 4 21) (end 4 22)) (probe (position 4 21))
    (reference (id (source (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::Q::ops"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_state_action_declaration_forms.md") (qualified-name "P::A")))))
    )
  )
)
~~~
