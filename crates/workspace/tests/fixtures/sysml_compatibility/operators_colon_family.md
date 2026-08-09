# META
~~~ini
description=Colon family operators
type=file
~~~
# SOURCE
~~~sysml
: :: :> ::> :>> :=
~~~
# TOKENS
~~~zig
Colon,ColonColon,ColonGt,ColonColonGt,ColonGtGt,ColonEq,EndOfFile,
~~~
# AST
~~~
(root
  (malformed))
~~~
# FORMAT
~~~sysml
: :: :> ::> :>> :=
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
