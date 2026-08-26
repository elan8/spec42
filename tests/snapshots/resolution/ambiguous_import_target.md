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
    import Shared::*;
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
        (range (start 1 11) (end 1 20))
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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 11) (end 1 20))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d49916dfa251a36d6a71e7b1ef1dc0d6df60d3d29fd0860ff0e2ec7e84929b98") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/A.sysml") (qualified-name "Shared"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/A.sysml") (qualified-name "Shared::Thing"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/B.sysml") (qualified-name "Shared"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/B.sysml") (qualified-name "Shared::Thing"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/Use.sysml") (qualified-name "Use"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/Use.sysml") (path (named (kind package) (name "Use")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "Shared") (import (shape namespace) (recursive false))))))
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
  (query (document "memory://snapshot/Use.sysml") (range (start 1 11) (end 1 20)) (probe (position 1 11))
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
