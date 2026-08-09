# META
~~~ini
description=Empty member (bare semicolon) at file level
type=file
~~~
# SOURCE
~~~sysml
; in v : SpeedVal
~~~
# TOKENS
~~~zig
Semicolon,KwIn,Ident,Colon,Ident,EndOfFile,
~~~
# AST
~~~
(root
  (malformed))
~~~
# FORMAT
~~~sysml
~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
~~~
# SMG
~~~
(model
  (namespace
    (not_implemented 'malformed')))
~~~
