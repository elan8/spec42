# META
~~~ini
description=Documentation node with malformed comment body should close the comment when formatting
type=file
~~~
# SOURCE
~~~sysml
alias Foo for Bar {
    doc /* unclosed comment
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_doc_malformed_comment_body.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 0 14) (end 0 17))
      )
      (diagnostic
        (severity error)
        (code "recovered_relationship_body_element")
        (source "parser")
        (range (start 1 4) (end 2 1))
      )
      (diagnostic
        (severity error)
        (code "missing_closing_brace")
        (source "parser")
        (range (start 2 1) (end 2 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:232b434e61af6751c54afb1533253422b1fbd4cdb4df3b95bc27fc3655f9afc9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_doc_malformed_comment_body.md") (qualified-name "Foo"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "Bar")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/fuzz_doc_malformed_comment_body.md") (qualified-name "Foo"))) (kind aliasBinding) (ordinal 0))
      (authored-target "Bar")
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/fuzz_doc_malformed_comment_body.md") (range (start 0 14) (end 0 17)) (probe (position 0 14))
    (reference (id (source (node (document "memory://snapshot/fuzz_doc_malformed_comment_body.md") (qualified-name "Foo"))) (kind aliasBinding) (ordinal 0) (authored-target "Bar")
      (outcome (status unresolved)))
    )
  )
)
~~~
