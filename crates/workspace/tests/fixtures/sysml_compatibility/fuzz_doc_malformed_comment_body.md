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
    doc /* unclosed comment */
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
  (status (skip (code "SMG-EMPTY-RECOVERY") (reason "parser recovery for non-empty source produced no typed semantic graph facts")))
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
