# META
~~~ini
description=Regular comments are tokens, notes are trivia
type=file
~~~
# SOURCE
~~~sysml
x /* comment */ // note
y
~~~
# TOKENS
~~~zig
Ident,RegularComment,LineComment,
Ident,EndOfFile,
~~~
# AST
~~~
(root
  (malformed))
~~~
# FORMAT
~~~sysml
x /* comment */ // note
y
~~~
# EXPECTED
~~~
parse.unexpected_token
~~~
# PROBLEMS
~~~
parse.unexpected_token
~~~
# SMG
~~~
(model
  (namespace
    (not_implemented 'malformed')))
~~~
