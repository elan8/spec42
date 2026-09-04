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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 25) (end 15 32))
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
  (publication (phase resolved) (completeness parse-recovery,unsupported-syntax) (has-evaluation true) (source-digest "blake3:39d8e81bc31ba9b62faf2db6d4162e3cba6dead832061af4f8253b4e9d9d0b69"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "a1")) (succession (reference "a2")))))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "a1::y")) (flowTarget (reference "a2::x")))))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a1"))) (kind kerml-step) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action1")))))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a2"))) (kind kerml-step) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action1")))))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E"))) (kind kerml-expression) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "x")))))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::I"))) (kind kerml-invariant) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Inter"))) (kind kerml-interaction) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Inter::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Inter::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P"))) (kind kerml-predicate) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "x")))))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in)))))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::b"))) (kind kerml-boolean-expression) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::s1"))) (kind kerml-step) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action1")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a1")))))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "a2")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a2")))))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0))
      (authored-target "a1::y")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::y")))))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0))
      (authored-target "a2::x")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::x")))))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Action1")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")))))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Action1")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")))))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E"))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E::x")))))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P"))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P::x")))))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::s1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Action1")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a1"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a2"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::s1"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::s1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::x"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::y"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a1"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a2"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E::x"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F::a"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F::result"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Inter::x"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Inter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Inter::y"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Inter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P::x"))) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::I"))) (state literal) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::b"))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")))
      (subtype (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a1")) (scopes any))
      (subtype (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a2")) (scopes any))
      (subtype (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::s1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::x")))
      (featured-by (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::y")))
      (featured-by (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind flow) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a1")))
      (featured-by (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container")))
      (type (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a2")))
      (featured-by (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container")))
      (type (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")) (scopes any))
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
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E"))) (outcome resolved) (operator "+" (literal (value (kind integer) (integer 1))) (feature-reference "x" (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E::x"))))))
  (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::I"))) (outcome resolved) (literal (value (kind boolean) (boolean true))))
  (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P"))) (outcome resolved) (feature-reference "x" (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P::x")))))
  (declaration (id (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::b"))) (outcome resolved) (literal (value (kind boolean) (boolean true))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_behaviors.md") (range (start 30 19) (end 30 21)) (probe (position 30 19))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a1")))))
    )
  )
  (query (document "memory://snapshot/coverage_behaviors.md") (range (start 30 27) (end 30 29)) (probe (position 30 27))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "a2")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a2")))))
    )
  )
  (query (document "memory://snapshot/coverage_behaviors.md") (range (start 31 13) (end 31 17)) (probe (position 31 13))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0) (authored-target "a1::y")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::y")))))
    )
  )
  (query (document "memory://snapshot/coverage_behaviors.md") (range (start 31 21) (end 31 25)) (probe (position 31 21))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (path (named (kind package) (name "BehaviorCoverage")) (named (kind class-def) (name "Container")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0) (authored-target "a2::x")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1::x")))))
    )
  )
  (query (document "memory://snapshot/coverage_behaviors.md") (range (start 28 18) (end 28 25)) (probe (position 28 18))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a1"))) (kind featureTyping) (ordinal 0) (authored-target "Action1")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")))))
    )
  )
  (query (document "memory://snapshot/coverage_behaviors.md") (range (start 29 18) (end 29 25)) (probe (position 29 18))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Container::a2"))) (kind featureTyping) (ordinal 0) (authored-target "Action1")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::Action1")))))
    )
  )
  (query (document "memory://snapshot/coverage_behaviors.md") (range (start 13 23) (end 13 24)) (probe (position 13 23))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E"))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::E::x")))))
    )
  )
  (query (document "memory://snapshot/coverage_behaviors.md") (range (start 10 32) (end 10 39)) (probe (position 10 32))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::F::result"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_behaviors.md") (range (start 15 34) (end 15 35)) (probe (position 15 34))
    (reference (id (source (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P"))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_behaviors.md") (qualified-name "BehaviorCoverage::P::x")))))
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
