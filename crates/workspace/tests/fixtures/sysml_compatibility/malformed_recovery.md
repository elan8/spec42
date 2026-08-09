# META
~~~ini
description=Malformed tokens with recovery
type=file
~~~
# SOURCE
~~~sysml
x ` y
~~~
# TOKENS
~~~zig
Ident,MalformedUnknownToken,Ident,EndOfFile,
~~~
# AST
~~~
(root
  (malformed))
~~~
# FORMAT
~~~sysml
x ` y
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
