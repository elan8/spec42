# META
~~~ini
description=The ConnectionUsage alternative connect (e1, e2, e3) recovers in a part definition body; n-ary ends parse on connection ... connect (...)
type=file
~~~
# SOURCE
~~~sysml
package Connections {
    part def A { port p; port r; }
    part def B { port q; }
    part def System {
        part a : A;
        part b : B;
        connect (a.p, b.q, a.r);
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/nary_bare_connect.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 1 17) (end 1 24))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 1 25) (end 1 32))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 2 17) (end 2 24))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "parser")
        (range (start 6 8) (end 7 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:07ebc8d1b47278d0f0e5b5fd9715cdd5e78c090e06ef0438492da8bf83c36555"))
  (declarations
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A::p"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A::r"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::B"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::B::q"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A")))))
    (reference (id (source (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::B")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::a"))) (target (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::b"))) (target (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A::p"))) (target (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A::r"))) (target (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::B::q"))) (target (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::B"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::a"))) (target (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::b"))) (target (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A")))
      (subtype (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::a")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A::p")))
      (featured-by (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A")))
    )
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A::r")))
      (featured-by (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A")))
    )
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::B")))
      (subtype (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::b")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::B::q")))
      (featured-by (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::B")))
    )
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::a")))
      (featured-by (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System")))
      (type (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A")) (source direct))
      (supertype (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::b")))
      (featured-by (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System")))
      (type (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::B")) (source direct))
      (supertype (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::B")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/nary_bare_connect.md") (range (start 4 17) (end 4 18)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::A")))))
    )
  )
  (query (document "memory://snapshot/nary_bare_connect.md") (range (start 5 17) (end 5 18)) (probe (position 5 17))
    (reference (id (source (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::System::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/nary_bare_connect.md") (qualified-name "Connections::B")))))
    )
  )
)
~~~
