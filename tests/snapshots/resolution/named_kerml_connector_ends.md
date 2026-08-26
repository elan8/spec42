# META
~~~ini
description=Named KerML connector ends own their reference-subsetting targets
type=file
observed_gap=The name before references declares an owned end Feature; only the following path is its ConnectorEnd target.
~~~
# SOURCE
~~~kerml
package Demo {
    assoc Link;

    class Context {
        feature actualSource;
        feature actualTarget;

        connector link : Link
            from [1] source references actualSource
            to [1] target references actualTarget;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind connector_end)
    (source "Demo::Context::link::source")
    (target "Demo::Context::actualSource")
    (provenance authored)
    (outcome resolved))
  (relationship
    (kind connector_end)
    (source "Demo::Context::link::target")
    (target "Demo::Context::actualTarget")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/named_kerml_connector_ends.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3774a499119a63cfe86903e92b0998680473e6e153441d157b7e2f1821a6faf3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::actualSource"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::actualTarget"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Link")))))
    (declaration (id (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1)) (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "actualSource")))))
    (declaration (id (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1)) (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "actualTarget")))))
    (declaration (id (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Link"))) (kind kerml-association) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link"))) (kind featureTyping) (ordinal 0))
      (authored-target "Link")
      (outcome (status resolved) (target (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Link")))))
    (reference (id (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link::source"))) (kind connectorEnd) (ordinal 0))
      (authored-target "actualSource")
      (outcome (status resolved) (target (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::actualSource")))))
    (reference (id (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link::target"))) (kind connectorEnd) (ordinal 0))
      (authored-target "actualTarget")
      (outcome (status resolved) (target (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::actualTarget")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link"))) (target (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Link"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link::source"))) (target (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::actualSource"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link::source"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link::target"))) (target (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::actualTarget"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link::target"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::actualSource"))) (target (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::actualTarget"))) (target (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link"))) (target (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link::source"))) (target (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link::target"))) (target (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::actualSource")))
      (featured-by (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context")))
    )
    (declaration (id (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::actualTarget")))
      (featured-by (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context")))
    )
    (declaration (id (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context")))
      (type (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Link")) (provenance authored))
      (effective-type (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Link")) (source direct))
      (supertype (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Link")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link::source")))
      (featured-by (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link")))
    )
    (declaration (id (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link::target")))
      (featured-by (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link")))
    )
    (declaration (id (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Link")))
      (subtype (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/named_kerml_connector_ends.md") (range (start 7 25) (end 7 29)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link"))) (kind featureTyping) (ordinal 0) (authored-target "Link")
      (outcome (status resolved) (target (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Link")))))
    )
  )
  (query (document "memory://snapshot/named_kerml_connector_ends.md") (range (start 8 39) (end 8 51)) (probe (position 8 39))
    (reference (id (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link::source"))) (kind connectorEnd) (ordinal 0) (authored-target "actualSource")
      (outcome (status resolved) (target (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::actualSource")))))
    )
  )
  (query (document "memory://snapshot/named_kerml_connector_ends.md") (range (start 9 37) (end 9 49)) (probe (position 9 37))
    (reference (id (source (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::link::target"))) (kind connectorEnd) (ordinal 0) (authored-target "actualTarget")
      (outcome (status resolved) (target (node (document "memory://snapshot/named_kerml_connector_ends.md") (qualified-name "Demo::Context::actualTarget")))))
    )
  )
)
~~~
