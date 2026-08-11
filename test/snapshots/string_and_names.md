# META
~~~ini
description=String literals and unrestricted names
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
"hello" 'world name' "with\nescapes"
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "string_and_names.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "sysml")
        (range (start 0 0) (end 0 36))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
StringValue,UnrestrictedName,StringValue,EndOfFile,
~~~
# AST
~~~
(root
  (malformed))
~~~
# EXPECTED
~~~
parse.unexpected_token
~~~
# PROBLEMS
~~~
parse.unexpected_token
~~~
# FORMAT
~~~sysml
"hello" 'world name' "with\nescapes"

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "856687fe21778474da104771454af27c8ebf93c2c76d7c64d89cbf60b604e870") (contract-version "canonical-resolution-v1"))
  (structure
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
