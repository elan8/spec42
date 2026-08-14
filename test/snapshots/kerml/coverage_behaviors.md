# META
~~~ini
description=Coverage: KerML behavior, step, function, expression, predicate, bool, inv, interaction, flow, succession flow
type=file
~~~
# SOURCE
~~~kerml
package BehaviorCoverage {
    behavior Action1 {
        in x;
        out y;
    }

    step s1 : Action1;

    function F {
        in a;
        return feature result : Integer;
    }

    expr E { in x; 1 + x }

    predicate P { in x : Boolean; x }

    bool b { true }

    inv I { true }
    inv false NegI { false }

    interaction Inter {
        in x;
        out y;
    }

    class Container {
        step a1 : Action1;
        step a2 : Action1;
        succession a1 then a2;
        flow a1.y to a2.x;
        succession flow sf from a1.y to a2.x;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/coverage_behaviors.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 32) (end 10 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 23) (end 13 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 25) (end 15 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 34) (end 15 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 20 4) (end 20 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 20 4) (end 20 28))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 28 8) (end 29 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 29 8) (end 30 8))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 30 8) (end 31 8))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 31 8) (end 32 8))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 32 8) (end 33 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:39d8e81bc31ba9b62faf2db6d4162e3cba6dead832061af4f8253b4e9d9d0b69") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::x"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::y"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E"))) (kind kerml-expression) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "x")))))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E::x"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F::a"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::I"))) (kind kerml-invariant) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Inter"))) (kind kerml-interaction) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Inter::x"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Inter::y"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P"))) (kind kerml-predicate) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "x")))))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P::x"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in)))))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::b"))) (kind kerml-boolean-expression) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::s1"))) (kind kerml-step) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action1")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E"))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P"))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::s1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Action1")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::s1"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::s1"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::I"))) (state literal) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::b"))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")))
      (subtype (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::s1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::x")))
      (featured-by (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::y")))
      (featured-by (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E::x")))
      (featured-by (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F::a")))
      (featured-by (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F::result")))
      (featured-by (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Inter::x")))
      (featured-by (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Inter")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Inter::y")))
      (featured-by (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Inter")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P::x")))
      (featured-by (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::s1")))
      (type (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_behaviors.md") (range (start 13 23) (end 13 24)) (probe (position 13 23))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E"))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_behaviors.md") (range (start 10 32) (end 10 39)) (probe (position 10 32))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F::result"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_behaviors.md") (range (start 15 34) (end 15 35)) (probe (position 15 34))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P"))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_behaviors.md") (range (start 15 25) (end 15 32)) (probe (position 15 25))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P::x"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_behaviors.md") (range (start 6 14) (end 6 21)) (probe (position 6 14))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::s1"))) (kind featureTyping) (ordinal 0) (authored-target "Action1")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")))))
    )
  )
)
~~~
