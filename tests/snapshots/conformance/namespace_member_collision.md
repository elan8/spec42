# META
~~~ini
description=A namespace reports the later of two members it cannot tell apart
type=file
~~~
# SOURCE
~~~sysml
package Collides {
    part def Shared;
    action def Shared;
    part def Distinct;
}
package Separate {
    part def Shared;
    part shared;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/namespace_member_collision.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 2 4) (end 2 22))
        (related-information
          (related
            (uri "memory://snapshot/namespace_member_collision.md")
            (range (start 1 4) (end 1 20))
          )
        )
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 7 4) (end 7 16))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9acc95f0ac446954e45170a27786044f4a002e6c5b1b108ca6ee563d06ba1775") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/namespace_member_collision.md") (qualified-name "Collides"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_collision.md") (qualified-name "Collides::Distinct"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_collision.md") (path (named (kind package) (name "Collides")) (named (kind part-def) (name "Shared"))))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_collision.md") (path (named (kind package) (name "Collides")) (named (kind action-def) (name "Shared"))))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_collision.md") (qualified-name "Separate"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_collision.md") (qualified-name "Separate::Shared"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_collision.md") (qualified-name "Separate::shared"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
