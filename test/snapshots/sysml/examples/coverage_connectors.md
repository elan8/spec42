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
    (element (id (node (document "d0") (qualified-name "A"))) (kind "part def") (name "A") (declared-name "A") (range (start (line 0) (character 0)) (end (line 0) (character 32))))
    (element (id (node (document "d0") (qualified-name "A::p1"))) (kind "port") (name "p1") (declared-name "p1") (range (start (line 0) (character 13)) (end (line 0) (character 21))) (parent (node (document "d0") (qualified-name "A"))))
    (element (id (node (document "d0") (qualified-name "A::p2"))) (kind "port") (name "p2") (declared-name "p2") (range (start (line 0) (character 22)) (end (line 0) (character 30))) (parent (node (document "d0") (qualified-name "A"))))
    (element (id (node (document "d0") (qualified-name "B"))) (kind "part def") (name "B") (declared-name "B") (range (start (line 1) (character 0)) (end (line 1) (character 32))))
    (element (id (node (document "d0") (qualified-name "B::q1"))) (kind "port") (name "q1") (declared-name "q1") (range (start (line 1) (character 13)) (end (line 1) (character 21))) (parent (node (document "d0") (qualified-name "B"))))
    (element (id (node (document "d0") (qualified-name "B::q2"))) (kind "port") (name "q2") (declared-name "q2") (range (start (line 1) (character 22)) (end (line 1) (character 30))) (parent (node (document "d0") (qualified-name "B"))))
    (element (id (node (document "d0") (qualified-name "System"))) (kind "part def") (name "System") (declared-name "System") (range (start (line 3) (character 0)) (end (line 3) (character 245))))
    (element (id (node (document "d0") (qualified-name "System::a"))) (kind "part") (name "a") (declared-name "a") (range (start (line 4) (character 4)) (end (line 4) (character 15))) (parent (node (document "d0") (qualified-name "System"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range (start (line 4) (character 13)) (end (line 4) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "System::b"))) (kind "part") (name "b") (declared-name "b") (range (start (line 5) (character 4)) (end (line 5) (character 15))) (parent (node (document "d0") (qualified-name "System"))) (authored (membership (kind Feature)) (relationships (typing (reference "B") (range (start (line 5) (character 13)) (end (line 5) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "System::engine"))) (kind "ref") (name "engine") (declared-name "engine") (range (start (line 13) (character 4)) (end (line 13) (character 24))) (parent (node (document "d0") (qualified-name "System"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range (start (line 13) (character 22)) (end (line 13) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "System::myA"))) (kind "part") (name "myA") (declared-name "myA") (range (start (line 14) (character 4)) (end (line 14) (character 28))) (parent (node (document "d0") (qualified-name "System"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range (start (line 14) (character 26)) (end (line 14) (character 27)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "System::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range (start (line 4) (character 13)) (end (line 4) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "A")))))
    (reference (id (source (node (document "d0") (qualified-name "System::b"))) (kind featureTyping) (ordinal 0)) (authored-target "B") (range (start (line 5) (character 13)) (end (line 5) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "B")))))
    (reference (id (source (node (document "d0") (qualified-name "System::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range (start (line 13) (character 22)) (end (line 13) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "A")))))
    (reference (id (source (node (document "d0") (qualified-name "System::myA"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range (start (line 14) (character 26)) (end (line 14) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "A")))))
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
