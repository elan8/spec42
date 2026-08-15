# META
~~~ini
description=KerML Simple Tests: Behaviors
type=file
~~~
# SOURCE
~~~kerml
package Behaviors {
    behavior A {
        in x;
        out y = b.y1;
        composite step b : B {
            in x1 = A::x;
        }
    }
    behavior B specializes A {
        in x1;
        out var y1;
    }
    class C {
        var z = A().y;
        step a : A;
        step b : B;
        binding z = a.y;
        flow a.y to b.x1;
    }
    abstract flow msg of C;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/behaviors.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 13 16) (end 13 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 16 8) (end 16 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_parser_construct")
        (source "semantic")
        (range (start 16 8) (end 16 24))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 17 8) (end 18 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 19 4) (end 19 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:68865699f91e36a67d4f07c6f249bb6a3e1bcf946c5621a2750834b306b37d8e") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::b"))) (kind kerml-step) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::b::x1"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "A::x")))))
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::x"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::y"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "b::y1")))))
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B"))) (kind kerml-behavior) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A")))))
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B::x1"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B::y1"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers var) (direction out)))
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::a"))) (kind kerml-step) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::b"))) (kind kerml-step) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::z"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (feature-value (kind bind)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B")))))
    (reference (id (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::b::x1"))) (kind expressionOperand) (ordinal 0))
      (authored-target "A::x")
      (outcome (status resolved) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::x")))))
    (reference (id (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::y"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "b::y1")
      (outcome (status resolved) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B::y1")))))
    (reference (id (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A")))))
    (reference (id (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A")))))
    (reference (id (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::b"))) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::b::x1"))) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::b::x1"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::y"))) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B::y1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::y"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B"))) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::a"))) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::b"))) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::b"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::b::x1"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A")))
      (subtype (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::a")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::b")))
      (featured-by (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A")))
      (type (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B")) (source direct))
      (supertype (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A")) (scopes any))
      (supertype (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::b::x1")))
      (featured-by (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::b")))
    )
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::x")))
      (featured-by (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A")))
    )
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::y")))
      (featured-by (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A")))
    )
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B")))
      (supertype (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::b")) (scopes any))
      (subtype (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::b")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B::x1")))
      (featured-by (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B")))
    )
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B::y1")))
      (featured-by (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B")))
    )
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::a")))
      (featured-by (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C")))
      (type (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A")) (source direct))
      (supertype (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::b")))
      (featured-by (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C")))
      (type (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B")) (source direct))
      (supertype (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A")) (scopes any))
      (supertype (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::z")))
      (featured-by (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/behaviors.md") (range (start 4 27) (end 4 28)) (probe (position 4 27))
    (reference (id (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B")))))
    )
  )
  (query (document "memory://snapshot/behaviors.md") (range (start 5 20) (end 5 24)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::b::x1"))) (kind expressionOperand) (ordinal 0) (authored-target "A::x")
      (outcome (status resolved) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::x")))))
    )
  )
  (query (document "memory://snapshot/behaviors.md") (range (start 3 16) (end 3 20)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A::y"))) (kind memberAccessOperand) (ordinal 0) (authored-target "b::y1")
      (outcome (status resolved) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B::y1")))))
    )
  )
  (query (document "memory://snapshot/behaviors.md") (range (start 8 27) (end 8 28)) (probe (position 8 27))
    (reference (id (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A")))))
    )
  )
  (query (document "memory://snapshot/behaviors.md") (range (start 14 17) (end 14 18)) (probe (position 14 17))
    (reference (id (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::A")))))
    )
  )
  (query (document "memory://snapshot/behaviors.md") (range (start 15 17) (end 15 18)) (probe (position 15 17))
    (reference (id (source (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::C::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/behaviors.md") (qualified-name "Behaviors::B")))))
    )
  )
)
~~~
