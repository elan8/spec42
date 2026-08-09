# META
~~~ini
description=Unclosed comment (missing */) at file level should be preserved
type=file
~~~
# SOURCE
~~~sysml
/* unclosed comment without closing marker
~~~
# TOKENS
~~~zig
MalformedRegularComment,EndOfFile,
~~~
# AST
~~~
(root
  (malformed))
~~~
# FORMAT
~~~sysml
/* unclosed comment without closing marker
~~~
# EXPECTED
~~~
tokenize.UnclosedRegularComment
~~~
# PROBLEMS
~~~
tokenize.UnclosedRegularComment
~~~
# SMG
~~~
(model
  (namespace
    (not_implemented 'malformed')))
~~~
