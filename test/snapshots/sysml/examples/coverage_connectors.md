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
  (document "coverage_connectors.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 7 4) (end 7 143))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "a2fc16301a164ff9516d6bf5db5efd33cd9b46ce86568a6e9ee914b5ea717bcd") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "A"))) (kind "part def") (name "A") (declared-name "A"))
    (element (id (node (document "d0") (qualified-name "A::p1"))) (kind "port") (name "p1") (declared-name "p1") (parent (node (document "d0") (qualified-name "A"))))
    (element (id (node (document "d0") (qualified-name "A::p2"))) (kind "port") (name "p2") (declared-name "p2") (parent (node (document "d0") (qualified-name "A"))))
    (element (id (node (document "d0") (qualified-name "B"))) (kind "part def") (name "B") (declared-name "B"))
    (element (id (node (document "d0") (qualified-name "B::q1"))) (kind "port") (name "q1") (declared-name "q1") (parent (node (document "d0") (qualified-name "B"))))
    (element (id (node (document "d0") (qualified-name "B::q2"))) (kind "port") (name "q2") (declared-name "q2") (parent (node (document "d0") (qualified-name "B"))))
    (element (id (node (document "d0") (qualified-name "System"))) (kind "part def") (name "System") (declared-name "System"))
    (element (id (node (document "d0") (qualified-name "System::a"))) (kind "part") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "System"))) (authored (membership (kind Feature)) (relationships (typing (reference "A")))))
    (element (id (node (document "d0") (qualified-name "System::b"))) (kind "part") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "System"))) (authored (membership (kind Feature)) (relationships (typing (reference "B")))))
    (element (id (node (document "d0") (qualified-name "System::engine"))) (kind "ref") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "System"))) (authored (membership (kind Feature)) (relationships (typing (reference "A")))))
    (element (id (node (document "d0") (qualified-name "System::myA"))) (kind "part") (name "myA") (declared-name "myA") (parent (node (document "d0") (qualified-name "System"))) (authored (membership (kind Feature)) (relationships (typing (reference "A")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "System::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "A")))))
    (reference (id (source (node (document "d0") (qualified-name "System::b"))) (kind featureTyping) (ordinal 0)) (authored-target "B") (outcome (status resolved) (target (node (document "d0") (qualified-name "B")))))
    (reference (id (source (node (document "d0") (qualified-name "System::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "A")))))
    (reference (id (source (node (document "d0") (qualified-name "System::myA"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "A")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "System::a"))) (target (node (document "d0") (qualified-name "A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "System::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "System::b"))) (target (node (document "d0") (qualified-name "B"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "System::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "System::engine"))) (target (node (document "d0") (qualified-name "A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "System::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "System::myA"))) (target (node (document "d0") (qualified-name "A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "System::myA"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 13) (end 4 14)) (probe (position 4 13))
      (reference
        (source (document "d0") (qualified-name "System::a"))
        (kind featureTyping) (ordinal 0) (authored-target "A")
        (range (start 4 13) (end 4 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "A") (range (start 0 0) (end 0 32)))
        )
      )
    )
    (query (range (start 5 13) (end 5 14)) (probe (position 5 13))
      (reference
        (source (document "d0") (qualified-name "System::b"))
        (kind featureTyping) (ordinal 0) (authored-target "B")
        (range (start 5 13) (end 5 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "B") (range (start 1 0) (end 1 32)))
        )
      )
    )
    (query (range (start 13 22) (end 13 23)) (probe (position 13 22))
      (reference
        (source (document "d0") (qualified-name "System::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "A")
        (range (start 13 22) (end 13 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "A") (range (start 0 0) (end 0 32)))
        )
      )
    )
    (query (range (start 14 26) (end 14 27)) (probe (position 14 26))
      (reference
        (source (document "d0") (qualified-name "System::myA"))
        (kind featureTyping) (ordinal 0) (authored-target "A")
        (range (start 14 26) (end 14 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "A") (range (start 0 0) (end 0 32)))
        )
      )
    )
  )
)
~~~
