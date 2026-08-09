# META
~~~ini
description=String literals and unrestricted names
type=file
~~~
# SOURCE
~~~sysml
"hello" 'world name' "with\nescapes"
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
# FORMAT
~~~sysml
"hello" 'world name' "with\nescapes"
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
