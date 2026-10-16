# META
~~~ini
description=SysML 8.2.2 DefinitionDeclaration rejects KerML disjoining while KerML 7.3.2.5 keeps the same type relationship valid
specification=OMG SysML 2.0 Language (formal/2026-03-02), 8.2.2 DefinitionDeclaration; OMG KerML 1.0, 7.3.2.5
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
libraries=none
~~~
# SOURCE
## invalid.sysml
~~~sysml
package DisjointTest {
    abstract action def A;
    abstract action def B disjoint from A;
    action def Good :> A;
}
~~~
## occurrence.sysml
~~~sysml
package OccurrenceDisjointTest {
    occurrence def A;
    occurrence def B disjoint from A;
}
~~~
## valid.kerml
~~~kerml
package Legal {
    abstract behavior A;
    abstract behavior B disjoint from A;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/invalid.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 2 4) (end 2 42))
      )
    )
  )
  (document "memory://snapshot/occurrence.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 2 4) (end 2 37))
      )
    )
  )
  (document "memory://snapshot/valid.kerml"
    (diagnostics
    )
  )
)
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "DisjointTest::Good") (target "DisjointTest::A") (provenance authored) (outcome resolved))
  (relationship (kind disjoining) (source "Legal::B") (target "Legal::A") (provenance authored) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/invalid.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 2 4) (end 2 42))
      )
    )
  )
  (document "memory://snapshot/occurrence.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 2 4) (end 2 37))
      )
    )
  )
  (document "memory://snapshot/valid.kerml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:fe379f1f870c638d4ff852697a9e959ca2be193bc3666c0a9ffd2dd219c2e235"))
  (declarations
    (declaration (id (node (document "memory://snapshot/invalid.sysml") (qualified-name "DisjointTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/invalid.sysml") (qualified-name "DisjointTest::A"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/invalid.sysml") (qualified-name "DisjointTest::Good"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A")))))
    (declaration (id (node (document "memory://snapshot/occurrence.sysml") (qualified-name "OccurrenceDisjointTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence.sysml") (qualified-name "OccurrenceDisjointTest::A"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/valid.kerml") (qualified-name "Legal"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/valid.kerml") (qualified-name "Legal::A"))) (kind kerml-behavior) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/valid.kerml") (qualified-name "Legal::B"))) (kind kerml-behavior) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (authored (membership (kind owning) (visibility default)) (relationships (disjoining (reference "A")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/invalid.sysml") (qualified-name "DisjointTest::Good"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/invalid.sysml") (qualified-name "DisjointTest::A")))))
    (reference (id (source (node (document "memory://snapshot/valid.kerml") (qualified-name "Legal::B"))) (kind disjoining) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/valid.kerml") (qualified-name "Legal::A")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/invalid.sysml") (qualified-name "DisjointTest::Good"))) (target (node (document "memory://snapshot/invalid.sysml") (qualified-name "DisjointTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/invalid.sysml") (qualified-name "DisjointTest::Good"))) (kind specialization) (ordinal 0)))
    (relationship (kind disjoining) (source (node (document "memory://snapshot/valid.kerml") (qualified-name "Legal::B"))) (target (node (document "memory://snapshot/valid.kerml") (qualified-name "Legal::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/valid.kerml") (qualified-name "Legal::B"))) (kind disjoining) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/invalid.sysml") (qualified-name "DisjointTest::A")))
      (subtype (node (document "memory://snapshot/invalid.sysml") (qualified-name "DisjointTest::Good")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/invalid.sysml") (qualified-name "DisjointTest::Good")))
      (supertype (node (document "memory://snapshot/invalid.sysml") (qualified-name "DisjointTest::A")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/valid.kerml") (qualified-name "Legal::B")))
      (set-operand (operator disjoint) (ordinal 0) (node (document "memory://snapshot/valid.kerml") (qualified-name "Legal::A")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/invalid.sysml") (range (start 3 23) (end 3 24)) (probe (position 3 23))
    (reference (id (source (node (document "memory://snapshot/invalid.sysml") (qualified-name "DisjointTest::Good"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/invalid.sysml") (qualified-name "DisjointTest::A")))))
    )
  )
  (query (document "memory://snapshot/valid.kerml") (range (start 2 38) (end 2 39)) (probe (position 2 38))
    (reference (id (source (node (document "memory://snapshot/valid.kerml") (qualified-name "Legal::B"))) (kind disjoining) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/valid.kerml") (qualified-name "Legal::A")))))
    )
  )
)
~~~
