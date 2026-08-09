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
