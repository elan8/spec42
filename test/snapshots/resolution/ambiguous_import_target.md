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
  (document "A.sysml"
    (diagnostics
    )
  )
  (document "B.sysml"
    (diagnostics
    )
  )
  (document "Use.sysml"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 4) (end 1 21))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_import_target")
        (source "semantic")
        (range (start 1 11) (end 1 17))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/A.sysml")
            (range (start 0 0) (end 0 34))
          )
          (related
            (uri "memory://snapshot/snapshot/B.sysml")
            (range (start 0 0) (end 0 34))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 2 17) (end 2 22))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/A.sysml")
            (range (start 0 17) (end 0 32))
          )
          (related
            (uri "memory://snapshot/snapshot/B.sysml")
            (range (start 0 17) (end 0 32))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1eab7aa02395b376bc91e3a7c5956222dc88b0da8a9f77368a7b142c279f8c23") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Shared"))) (kind "package") (name "Shared") (declared-name "Shared"))
    (element (id (node (document "d0") (qualified-name "Shared::Thing"))) (kind "part def") (name "Thing") (declared-name "Thing") (parent (node (document "d0") (qualified-name "Shared"))))
    (element (id (node (document "d1") (qualified-name "Shared"))) (kind "package") (name "Shared") (declared-name "Shared"))
    (element (id (node (document "d1") (qualified-name "Shared::Thing"))) (kind "part def") (name "Thing") (declared-name "Thing") (parent (node (document "d1") (qualified-name "Shared"))))
    (element (id (node (document "d2") (qualified-name "Use"))) (kind "package") (name "Use") (declared-name "Use"))
    (element (id (node (document "d2") (qualified-name "Use::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d2") (qualified-name "Use"))) (authored (membership (kind Import) (import (reference "Shared::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d2") (qualified-name "Use::usage"))) (kind "part") (name "usage") (declared-name "usage") (parent (node (document "d2") (qualified-name "Use"))) (authored (membership (kind Feature)) (relationships (typing (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "d2") (qualified-name "Use::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Shared::*") (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "Shared")) (node (document "d1") (qualified-name "Shared")))) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-ambiguous)))
    (reference (id (source (node (document "d2") (qualified-name "Use::usage"))) (kind featureTyping) (ordinal 0)) (authored-target "Thing") (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "Shared::Thing")) (node (document "d1") (qualified-name "Shared::Thing")))))
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
  (document "d2"
    (query (range (start 2 17) (end 2 22)) (probe (position 2 17))
      (reference
        (source (document "d2") (qualified-name "Use::usage"))
        (kind featureTyping) (ordinal 0) (authored-target "Thing")
        (range (start 2 17) (end 2 22))
        (outcome (status ambiguous)
          (target (document "d0") (qualified-name "Shared::Thing") (range (start 0 17) (end 0 32)))
          (target (document "d1") (qualified-name "Shared::Thing") (range (start 0 17) (end 0 32)))
        )
      )
    )
    (query (range (start 1 11) (end 1 17)) (probe (position 1 11))
      (reference
        (source (document "d2") (qualified-name "Use::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Shared::*")
        (range (start 1 11) (end 1 17))
        (outcome (status ambiguous)
          (target (document "d0") (qualified-name "Shared") (range (start 0 0) (end 0 34)))
          (target (document "d1") (qualified-name "Shared") (range (start 0 0) (end 0 34)))
        )
      )
    )
  )
)
~~~
