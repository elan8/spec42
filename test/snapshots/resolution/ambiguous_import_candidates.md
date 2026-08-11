# META
~~~ini
description=Ambiguous imported type preserves ordered candidate locations
type=file
~~~
# SOURCE
~~~sysml
package A { part def Thing; }
package B { part def Thing; }
package Use {
    import A::*;
    import B::*;
    part usage : Thing;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "ambiguous_import_candidates.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 3 4) (end 3 16))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 5 17) (end 5 22))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/ambiguous_import_candidates.md")
            (range (start 0 12) (end 0 27))
          )
          (related
            (uri "memory://snapshot/snapshot/ambiguous_import_candidates.md")
            (range (start 1 12) (end 1 27))
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "798fd92f6b46e161d670380ca7f30fa3627a058ff750511daf82acc8828a5afa") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "A"))) (kind "package") (name "A") (declared-name "A"))
    (element (id (node (document "d0") (qualified-name "A::Thing"))) (kind "part def") (name "Thing") (declared-name "Thing") (parent (node (document "d0") (qualified-name "A"))))
    (element (id (node (document "d0") (qualified-name "B"))) (kind "package") (name "B") (declared-name "B"))
    (element (id (node (document "d0") (qualified-name "B::Thing"))) (kind "part def") (name "Thing") (declared-name "Thing") (parent (node (document "d0") (qualified-name "B"))))
    (element (id (node (document "d0") (qualified-name "Use"))) (kind "package") (name "Use") (declared-name "Use"))
    (element (id (node (document "d0") (qualified-name "Use::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Use"))) (authored (membership (kind Import) (import (reference "A::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Use::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Use"))) (authored (membership (kind Import) (import (reference "B::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Use::usage"))) (kind "part") (name "usage") (declared-name "usage") (parent (node (document "d0") (qualified-name "Use"))) (authored (membership (kind Feature)) (relationships (typing (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Use::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "A::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "A")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "Use::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "B::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "B")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "Use::usage"))) (kind featureTyping) (ordinal 0)) (authored-target "Thing") (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "A::Thing")) (node (document "d0") (qualified-name "B::Thing")))))
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
    (query (range (start 3 11) (end 3 12)) (probe (position 3 11))
      (reference
        (source (document "d0") (qualified-name "Use::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "A::*")
        (range (start 3 11) (end 3 12))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "A") (range (start 0 0) (end 0 29)))
        )
      )
    )
    (query (range (start 4 11) (end 4 12)) (probe (position 4 11))
      (reference
        (source (document "d0") (qualified-name "Use::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "B::*")
        (range (start 4 11) (end 4 12))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "B") (range (start 1 0) (end 1 29)))
        )
      )
    )
    (query (range (start 5 17) (end 5 22)) (probe (position 5 17))
      (reference
        (source (document "d0") (qualified-name "Use::usage"))
        (kind featureTyping) (ordinal 0) (authored-target "Thing")
        (range (start 5 17) (end 5 22))
        (outcome (status ambiguous)
          (target (document "d0") (qualified-name "A::Thing") (range (start 0 12) (end 0 27)))
          (target (document "d0") (qualified-name "B::Thing") (range (start 1 12) (end 1 27)))
        )
      )
    )
  )
)
~~~
