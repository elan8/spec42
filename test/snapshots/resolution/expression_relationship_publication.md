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
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 0 46) (end 0 61))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:30c1e822853044cbfc7047aefcaddb7a4f69d092dae5e529c4d2a7c33868bf09") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System::a"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/expression_relationship_publication.md") (qualified-name "M::System::b"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
