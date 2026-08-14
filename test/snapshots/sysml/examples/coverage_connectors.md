# META
~~~ini
description=Coverage: Connector from/to forms, binding connector variants, connector specializations
type=file
~~~
# SOURCE
~~~sysml
part def A { port p1; port p2; }
part def B { port q1; port q2; }

part def System {
    part a : A;
    part b : B;

    connector c1 from a.p1 to b.q1;
    connector c2 :> c1 from a.p2 to b.q2;

    binding b1 of a.p1 = b.q1;
    binding of a.p2 = b.q2;

    ref part engine : A;
    individual part myA : A;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/coverage_connectors.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 7 4) (end 13 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:161429c0dd614f928a85292a7c7f59528ada4f3eaebf79d2bb8facd801568277") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "A"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "A::p1"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "A::p2"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "B"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "B::q1"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "B::q2"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B"))))
    (declaration (id (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::engine"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::myA"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "A")))))
    (reference (id (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "B")))))
    (reference (id (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "A")))))
    (reference (id (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::myA"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "A")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::a"))) (target (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::b"))) (target (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::engine"))) (target (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::myA"))) (target (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::myA"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::a")))
      (supertype (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::b")))
      (supertype (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "B")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::engine")))
      (supertype (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::myA")))
      (supertype (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "A")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_connectors.md") (range (start 4 13) (end 4 14)) (probe (position 4 13))
    (reference (id (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "A")))))
  )
  (query (document "memory://snapshot/coverage_connectors.md") (range (start 5 13) (end 5 14)) (probe (position 5 13))
    (reference (id (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "B")))))
  )
  (query (document "memory://snapshot/coverage_connectors.md") (range (start 13 22) (end 13 23)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::engine"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "A")))))
  )
  (query (document "memory://snapshot/coverage_connectors.md") (range (start 14 26) (end 14 27)) (probe (position 14 26))
    (reference (id (source (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "System::myA"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_connectors.md") (qualified-name "A")))))
  )
)
~~~
