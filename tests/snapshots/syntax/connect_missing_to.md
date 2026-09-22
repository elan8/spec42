# META
~~~ini
description=BinaryConnectorPart requires to between ends; connect with a single end must recover
type=file
~~~
# SOURCE
~~~sysml
package Connections {
    part def A { port p; }
    part def System {
        part a : A;
        connect a.p;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/connect_missing_to.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 1 17) (end 1 24))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "parser")
        (range (start 4 8) (end 5 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:7d0ff2a8761d3018bcc7b6b1ca419bf73a05b8d92f3ca8c34cbb008f4a935a9b"))
  (declarations
    (declaration (id (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::A"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::A::p"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::System::a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::System::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::A")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::System::a"))) (target (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::System::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::A::p"))) (target (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::System::a"))) (target (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::System"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::A")))
      (subtype (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::System::a")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::A::p")))
      (featured-by (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::A")))
    )
    (declaration (id (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::System::a")))
      (featured-by (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::System")))
      (type (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::A")) (source direct))
      (supertype (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::A")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/connect_missing_to.md") (range (start 3 17) (end 3 18)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::System::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connect_missing_to.md") (qualified-name "Connections::A")))))
    )
  )
)
~~~
