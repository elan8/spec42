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
