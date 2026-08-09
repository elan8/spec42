# META
~~~ini
description=Wildcard import statement
type=file
~~~
# SOURCE
~~~sysml
import ScalarValues::*;
~~~
# TOKENS
~~~zig
KwImport,Ident,ColonColon,Star,Semicolon,EndOfFile,
~~~
# AST
~~~
(root
  (import_decl 'ScalarValues::*'))
~~~
# FORMAT
~~~sysml
import ScalarValues::*;

~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "import") (id (node (document "d0") (qualified-name "*"))) (name "*") (declared-name "*"))
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "import_wildcard.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 0) (end 0 23))
      )
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 0 0) (end 0 23))
      )
    )
  )
)
~~~
