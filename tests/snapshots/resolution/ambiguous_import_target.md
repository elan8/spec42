# META
~~~ini
description=Ambiguous namespace import preserves category-specific diagnostics
type=multi
~~~
# SOURCE
## A.sysml
~~~sysml
package Shared { part def Thing; }
~~~
## B.sysml
~~~sysml
package Shared { part def Thing; }
~~~
## Use.sysml
~~~sysml
package Use {
    private import Shared::*;
    part usage : Thing;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/A.sysml"
    (diagnostics
    )
  )
  (document "memory://snapshot/B.sysml"
    (diagnostics
    )
  )
  (document "memory://snapshot/Use.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "ambiguous_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 28))
        (related-information
          (related
            (uri "memory://snapshot/A.sysml")
            (range (start 0 0) (end 0 34))
          )
          (related
            (uri "memory://snapshot/B.sysml")
            (range (start 0 0) (end 0 34))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 19) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 17) (end 2 22))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:50c5ebf65a80e791c35551755973c0c358888c592b86cbb38f019db1d03f028a"))
  (declarations
    (declaration (id (node (document "memory://snapshot/A.sysml") (qualified-name "Shared"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/A.sysml") (qualified-name "Shared::Thing"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/B.sysml") (qualified-name "Shared"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/B.sysml") (qualified-name "Shared::Thing"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/Use.sysml") (qualified-name "Use"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/Use.sysml") (path (named (kind package) (name "Use")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Shared") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/Use.sysml") (qualified-name "Use::usage"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/Use.sysml") (path (named (kind package) (name "Use")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Shared")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/A.sysml") (qualified-name "Shared")) (node (document "memory://snapshot/B.sysml") (qualified-name "Shared")))))
    (reference (id (source (node (document "memory://snapshot/Use.sysml") (qualified-name "Use::usage"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
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
  (query (document "memory://snapshot/Use.sysml") (range (start 1 19) (end 1 28)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/Use.sysml") (path (named (kind package) (name "Use")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Shared")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/A.sysml") (qualified-name "Shared")) (node (document "memory://snapshot/B.sysml") (qualified-name "Shared")))))
    )
  )
  (query (document "memory://snapshot/Use.sysml") (range (start 2 17) (end 2 22)) (probe (position 2 17))
    (reference (id (source (node (document "memory://snapshot/Use.sysml") (qualified-name "Use::usage"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status unresolved)))
    )
  )
)
~~~
