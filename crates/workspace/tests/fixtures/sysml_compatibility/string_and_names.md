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
