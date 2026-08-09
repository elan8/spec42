# META
~~~ini
description=Unclosed comment with special characters should be preserved
type=file
~~~
# SOURCE
~~~sysml
/* isio . /% #ato
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
/* isio . /% #ato
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
