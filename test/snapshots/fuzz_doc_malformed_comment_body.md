# META
~~~ini
description=Documentation node with malformed comment body should close the comment when formatting
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
alias Foo for Bar {
    doc /* unclosed comment
}
~~~
# TOKENS
~~~zig
KwAlias,Ident,KwFor,Ident,OpenCurly,
KwDoc,MalformedRegularComment,EndOfFile,
~~~
# AST
~~~
(root
  (alias_member 'Foo' for 'Bar'
    (documentation)))
~~~
# FORMAT
~~~sysml
alias Foo for Bar {
    doc /* unclosed comment
}
~~~
# EXPECTED
~~~
tokenize.UnclosedRegularComment
parse.expected_close_curly
~~~
# PROBLEMS
~~~
tokenize.UnclosedRegularComment
parse.expected_close_curly
~~~
# SMG
~~~
(semantic-graph
  (containment
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_doc_malformed_comment_body.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "sysml")
        (range (start 0 0) (end 0 19))
      )
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "sysml")
        (range (start 1 4) (end 1 27))
      )
      (diagnostic
        (severity error)
        (code "unexpected_closing_brace")
        (source "sysml")
        (range (start 2 0) (end 2 1))
      )
    )
  )
)
~~~
