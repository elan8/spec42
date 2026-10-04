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
        (severity warning)
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d7a81eaaed42a45c3f0092b123723fc6aac542b25bc46ebd6bf0318d46452820"))
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
