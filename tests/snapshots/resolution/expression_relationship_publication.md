# META
~~~ini
description=Expression relationship endpoints are resolved at publication
type=file
~~~
# SOURCE
~~~sysml
package M { part def System { part a; part b; connect a to b; } }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/expression_relationship_publication.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 0 30) (end 0 37))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 0 38) (end 0 45))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:30c1e822853044cbfc7047aefcaddb7a4f69d092dae5e529c4d2a7c33868bf09") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/expression_relationship_publication.md") (path (named (kind package) (name "M")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "a")) (connectorEnd (reference "b")))))
    (declaration (id (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System::a"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System::b"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/expression_relationship_publication.md") (path (named (kind package) (name "M")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System::a")))))
    (reference (id (source (node (document "memory://snapshot/expression_relationship_publication.md") (path (named (kind package) (name "M")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System::b")))))
  )
  (relationships
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/expression_relationship_publication.md") (path (named (kind package) (name "M")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/expression_relationship_publication.md") (path (named (kind package) (name "M")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/expression_relationship_publication.md") (path (named (kind package) (name "M")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/expression_relationship_publication.md") (path (named (kind package) (name "M")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/expression_relationship_publication.md") (path (named (kind package) (name "M")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System::a"))) (target (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System::b"))) (target (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/expression_relationship_publication.md") (path (named (kind package) (name "M")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System")))
    )
    (declaration (id (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System::a")))
      (featured-by (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System")))
    )
    (declaration (id (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System::b")))
      (featured-by (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/expression_relationship_publication.md") (range (start 0 54) (end 0 55)) (probe (position 0 54))
    (reference (id (source (node (document "memory://snapshot/expression_relationship_publication.md") (path (named (kind package) (name "M")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System::a")))))
    )
  )
  (query (document "memory://snapshot/expression_relationship_publication.md") (range (start 0 59) (end 0 60)) (probe (position 0 59))
    (reference (id (source (node (document "memory://snapshot/expression_relationship_publication.md") (path (named (kind package) (name "M")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System::b")))))
    )
  )
)
~~~
