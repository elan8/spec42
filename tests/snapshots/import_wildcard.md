# META
~~~ini
description=Wildcard import statement
type=file
~~~
# SOURCE
~~~sysml
import ScalarValues::*;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/import_wildcard.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 0 7) (end 0 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 7) (end 0 22))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:fe29f9182e5693634762d87d29e4d8620020032c1f2a5755fe0ca5c81a1bfe85") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/import_wildcard.md") (path (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/import_wildcard.md") (path (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/import_wildcard.md") (range (start 0 7) (end 0 22)) (probe (position 0 7))
    (reference (id (source (node (document "memory://snapshot/import_wildcard.md") (path (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
)
~~~
