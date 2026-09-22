# META
~~~ini
description=KerML connector from/to and binding of are not SysML ConnectionUsage or BindingConnectorAsUsage and must recover
type=file
~~~
# SOURCE
~~~sysml
package Connections {
    part def A { port p; }
    part def B { port q; }
    part def System {
        part a : A;
        part b : B;
        connector c1 from a.p to b.q;
        binding b1 of a.p = b.q;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_connector_spelling_in_sysml.md"
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
        (range (start 2 17) (end 2 24))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 6 8) (end 8 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:58d0ca8dbda9e0350dda7749a222dc045f5d04344c50a6ca2bfc3a329a34c8a2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::A"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::A::p"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::B"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::B::q"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::B")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::a"))) (target (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::b"))) (target (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::A::p"))) (target (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::B::q"))) (target (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::B"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::a"))) (target (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::b"))) (target (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::A")))
      (subtype (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::a")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::A::p")))
      (featured-by (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::A")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::B")))
      (subtype (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::b")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::B::q")))
      (featured-by (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::B")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::a")))
      (featured-by (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System")))
      (type (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::A")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::b")))
      (featured-by (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System")))
      (type (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::B")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::B")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (range (start 4 17) (end 4 18)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (range (start 5 17) (end 5 18)) (probe (position 5 17))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::System::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_spelling_in_sysml.md") (qualified-name "Connections::B")))))
    )
  )
)
~~~
