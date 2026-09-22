# META
~~~ini
description=InterfaceDefinition takes DefinitionDeclaration and InterfaceBody, not a connect clause
type=file
~~~
# SOURCE
~~~sysml
package Interfaces {
    port def Power;
    part def A { port p : Power; }
    part def B { port q : Power; }
    part def System {
        part a : A;
        part b : B;
        interface def PowerLink connect a.p to b.q;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/interface_def_cannot_connect.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 2 17) (end 2 32))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 3 17) (end 3 32))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "parser")
        (range (start 7 8) (end 8 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:ba4b1cc4383abfda6548927c9edc7718d3f005453f01730cbc5e505f884ccdb4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A::p"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Power")))))
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B::q"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Power")))))
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::Power"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::Power")))))
    (reference (id (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B::q"))) (kind featureTyping) (ordinal 0))
      (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::Power")))))
    (reference (id (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A")))))
    (reference (id (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A::p"))) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::Power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B::q"))) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::Power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B::q"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::a"))) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::b"))) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A::p"))) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B::q"))) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::a"))) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::b"))) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A")))
      (subtype (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::a")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A::p")))
      (featured-by (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A")))
      (type (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::Power")) (provenance authored))
      (effective-type (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::Power")) (source direct))
      (supertype (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::Power")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B")))
      (subtype (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::b")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B::q")))
      (featured-by (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B")))
      (type (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::Power")) (provenance authored))
      (effective-type (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::Power")) (source direct))
      (supertype (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::Power")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::Power")))
      (subtype (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A::p")) (scopes any))
      (subtype (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B::q")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::a")))
      (featured-by (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System")))
      (type (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A")) (source direct))
      (supertype (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::b")))
      (featured-by (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System")))
      (type (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B")) (source direct))
      (supertype (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/interface_def_cannot_connect.md") (range (start 2 26) (end 2 31)) (probe (position 2 26))
    (reference (id (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A::p"))) (kind featureTyping) (ordinal 0) (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::Power")))))
    )
  )
  (query (document "memory://snapshot/interface_def_cannot_connect.md") (range (start 3 26) (end 3 31)) (probe (position 3 26))
    (reference (id (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B::q"))) (kind featureTyping) (ordinal 0) (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::Power")))))
    )
  )
  (query (document "memory://snapshot/interface_def_cannot_connect.md") (range (start 5 17) (end 5 18)) (probe (position 5 17))
    (reference (id (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::A")))))
    )
  )
  (query (document "memory://snapshot/interface_def_cannot_connect.md") (range (start 6 17) (end 6 18)) (probe (position 6 17))
    (reference (id (source (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::System::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_def_cannot_connect.md") (qualified-name "Interfaces::B")))))
    )
  )
)
~~~
