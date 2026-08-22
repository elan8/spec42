# META
~~~ini
description=KerML Binding Connector: bind keyword and per-end multiplicities
type=file
~~~
# SOURCE
~~~kerml
package P {
    class C {
        feature x;
        feature y;
        feature startShot;
        feature endShot;
        feature baseEdges;

        binding [1] bind [0..*] x = [0..*] y;
        binding b bind lhs = rhs;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/binding_connector_bind_kw.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 8 8) (end 9 8))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 9 8) (end 10 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:934a6cefbf30848d75428ac15e460810e90dd4066ce9e38abcbdd0d519da4e8d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::baseEdges"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::endShot"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::startShot"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::baseEdges"))) (target (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::endShot"))) (target (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::startShot"))) (target (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::x"))) (target (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::y"))) (target (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::baseEdges")))
      (featured-by (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C")))
    )
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::endShot")))
      (featured-by (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C")))
    )
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::startShot")))
      (featured-by (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C")))
    )
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::x")))
      (featured-by (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C")))
    )
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::y")))
      (featured-by (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
