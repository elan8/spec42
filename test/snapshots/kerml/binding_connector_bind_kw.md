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
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 23) (end 9 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 29) (end 9 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:934a6cefbf30848d75428ac15e460810e90dd4066ce9e38abcbdd0d519da4e8d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "x")) (bindTarget (reference "y")))))
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 1))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "lhs")) (bindTarget (reference "rhs")))))
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::baseEdges"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::endShot"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::startShot"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::x")))))
    (reference (id (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 1))))) (kind bindSource) (ordinal 0))
      (authored-target "lhs")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::y")))))
    (reference (id (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 1))))) (kind bindTarget) (ordinal 0))
      (authored-target "rhs")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind bindSource) (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0)))
    (relationship (kind bindTarget) (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C")))
    )
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C")))
    )
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
  (query (document "memory://snapshot/binding_connector_bind_kw.md") (range (start 8 32) (end 8 33)) (probe (position 8 32))
    (reference (id (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::x")))))
    )
  )
  (query (document "memory://snapshot/binding_connector_bind_kw.md") (range (start 9 23) (end 9 26)) (probe (position 9 23))
    (reference (id (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 1))))) (kind bindSource) (ordinal 0) (authored-target "lhs")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/binding_connector_bind_kw.md") (range (start 8 43) (end 8 44)) (probe (position 8 43))
    (reference (id (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P::C::y")))))
    )
  )
  (query (document "memory://snapshot/binding_connector_bind_kw.md") (range (start 9 29) (end 9 32)) (probe (position 9 29))
    (reference (id (source (node (document "memory://snapshot/binding_connector_bind_kw.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "C")) (anonymous (kind bind) (ordinal 1))))) (kind bindTarget) (ordinal 0) (authored-target "rhs")
      (outcome (status unresolved)))
    )
  )
)
~~~
