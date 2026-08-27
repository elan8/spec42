# META
~~~ini
description=Connector definitions with references in ends
type=kerml
~~~
# SOURCE
~~~kerml
class A {
	feature self : A;
	feature this : A;
	connector :HappensDuring
		from [1] self references self
		to [1] this references this;
	connector :InsideOf
		from [0..*] smallerOccurrence references elements
		to [1] largerOccurrence references union;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/connector_references.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 12) (end 3 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 12) (end 6 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 43) (end 7 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 37) (end 8 42))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:001058ec9e6d1bb0b353852f260ad84da63f109f327985bcc0021860a6133a43"))
  (declarations
    (declaration (id (node (document "memory://snapshot/connector_references.md") (qualified-name "A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensDuring")))))
    (declaration (id (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1))))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InsideOf")))))
    (declaration (id (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1)) (named (kind kerml-feature) (name "largerOccurrence"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1)) (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "union")))))
    (declaration (id (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)) (named (kind kerml-feature) (name "self"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1)) (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "self")))))
    (declaration (id (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1)) (named (kind kerml-feature) (name "smallerOccurrence"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded)) (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "elements")))))
    (declaration (id (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)) (named (kind kerml-feature) (name "this"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1)) (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "this")))))
    (declaration (id (node (document "memory://snapshot/connector_references.md") (qualified-name "A::self"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/connector_references.md") (qualified-name "A::this"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensDuring")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "InsideOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1)) (named (kind kerml-feature) (name "largerOccurrence"))))) (kind connectorEnd) (ordinal 0))
      (authored-target "union")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)) (named (kind kerml-feature) (name "self"))))) (kind connectorEnd) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A::self")))))
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1)) (named (kind kerml-feature) (name "smallerOccurrence"))))) (kind connectorEnd) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)) (named (kind kerml-feature) (name "this"))))) (kind connectorEnd) (ordinal 0))
      (authored-target "this")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A::this")))))
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (qualified-name "A::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A")))))
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (qualified-name "A::this"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A")))))
  )
  (relationships
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)) (named (kind kerml-feature) (name "self"))))) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)) (named (kind kerml-feature) (name "self"))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)) (named (kind kerml-feature) (name "this"))))) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A::this"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)) (named (kind kerml-feature) (name "this"))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connector_references.md") (qualified-name "A::self"))) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_references.md") (qualified-name "A::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connector_references.md") (qualified-name "A::this"))) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_references.md") (qualified-name "A::this"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0))))) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1))))) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1)) (named (kind kerml-feature) (name "largerOccurrence"))))) (target (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)) (named (kind kerml-feature) (name "self"))))) (target (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1)) (named (kind kerml-feature) (name "smallerOccurrence"))))) (target (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)) (named (kind kerml-feature) (name "this"))))) (target (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_references.md") (qualified-name "A::self"))) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_references.md") (qualified-name "A::this"))) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/connector_references.md") (qualified-name "A")))
      (subtype (node (document "memory://snapshot/connector_references.md") (qualified-name "A::self")) (scopes any))
      (subtype (node (document "memory://snapshot/connector_references.md") (qualified-name "A::this")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)))))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/connector_references.md") (qualified-name "A")))
    )
    (declaration (id (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1)))))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/connector_references.md") (qualified-name "A")))
    )
    (declaration (id (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1)) (named (kind kerml-feature) (name "largerOccurrence")))))
      (featured-by (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)) (named (kind kerml-feature) (name "self")))))
      (featured-by (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1)) (named (kind kerml-feature) (name "smallerOccurrence")))))
      (featured-by (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)) (named (kind kerml-feature) (name "this")))))
      (featured-by (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/connector_references.md") (qualified-name "A::self")))
      (featured-by (node (document "memory://snapshot/connector_references.md") (qualified-name "A")))
      (type (node (document "memory://snapshot/connector_references.md") (qualified-name "A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_references.md") (qualified-name "A")) (source direct))
      (supertype (node (document "memory://snapshot/connector_references.md") (qualified-name "A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_references.md") (qualified-name "A::this")))
      (featured-by (node (document "memory://snapshot/connector_references.md") (qualified-name "A")))
      (type (node (document "memory://snapshot/connector_references.md") (qualified-name "A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_references.md") (qualified-name "A")) (source direct))
      (supertype (node (document "memory://snapshot/connector_references.md") (qualified-name "A")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/connector_references.md") (range (start 3 12) (end 3 25)) (probe (position 3 12))
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "HappensDuring")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connector_references.md") (range (start 6 12) (end 6 20)) (probe (position 6 12))
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "InsideOf")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connector_references.md") (range (start 8 37) (end 8 42)) (probe (position 8 37))
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1)) (named (kind kerml-feature) (name "largerOccurrence"))))) (kind connectorEnd) (ordinal 0) (authored-target "union")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connector_references.md") (range (start 4 27) (end 4 31)) (probe (position 4 27))
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)) (named (kind kerml-feature) (name "self"))))) (kind connectorEnd) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A::self")))))
    )
  )
  (query (document "memory://snapshot/connector_references.md") (range (start 7 43) (end 7 51)) (probe (position 7 43))
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 1)) (named (kind kerml-feature) (name "smallerOccurrence"))))) (kind connectorEnd) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connector_references.md") (range (start 5 25) (end 5 29)) (probe (position 5 25))
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (path (named (kind class-def) (name "A")) (anonymous (kind kerml-connector) (ordinal 0)) (named (kind kerml-feature) (name "this"))))) (kind connectorEnd) (ordinal 0) (authored-target "this")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A::this")))))
    )
  )
  (query (document "memory://snapshot/connector_references.md") (range (start 1 16) (end 1 17)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (qualified-name "A::self"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A")))))
    )
  )
  (query (document "memory://snapshot/connector_references.md") (range (start 2 16) (end 2 17)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/connector_references.md") (qualified-name "A::this"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_references.md") (qualified-name "A")))))
    )
  )
)
~~~
