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
  (document "import_wildcard.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 0 0) (end 0 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 7) (end 0 19))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
import ScalarValues::*;

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f04fc78cd1d8b899634a177ab1963bcf632cdd3c7e6d4a5b9502bbf39b983268") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "*"))) (kind "import") (name "*") (declared-name "*") (authored (membership (kind Import) (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 0 7) (end 0 19)) (probe (position 0 7))
      (reference
        (source (document "d0") (qualified-name "*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 0 7) (end 0 19))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
