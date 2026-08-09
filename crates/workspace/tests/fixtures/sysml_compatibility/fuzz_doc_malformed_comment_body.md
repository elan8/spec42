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
(model
  (namespace
    (alias_member 'Foo' -> 'Bar'[unresolved])))
~~~
