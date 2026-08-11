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
# FORMAT
~~~sysml
package A { part def Thing; }
package B { part def Thing; }
package Use {
    import A::*;
    import B::*;
    part usage : Thing;
}

~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "798fd92f6b46e161d670380ca7f30fa3627a058ff750511daf82acc8828a5afa") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "A"))) (kind "package") (name "A") (declared-name "A") (range (start (line 0) (character 0)) (end (line 0) (character 29))))
    (element (id (node (document "d0") (qualified-name "A::Thing"))) (kind "part def") (name "Thing") (declared-name "Thing") (range (start (line 0) (character 12)) (end (line 0) (character 27))) (parent (node (document "d0") (qualified-name "A"))))
    (element (id (node (document "d0") (qualified-name "B"))) (kind "package") (name "B") (declared-name "B") (range (start (line 1) (character 0)) (end (line 1) (character 29))))
    (element (id (node (document "d0") (qualified-name "B::Thing"))) (kind "part def") (name "Thing") (declared-name "Thing") (range (start (line 1) (character 12)) (end (line 1) (character 27))) (parent (node (document "d0") (qualified-name "B"))))
    (element (id (node (document "d0") (qualified-name "Use"))) (kind "package") (name "Use") (declared-name "Use") (range (start (line 2) (character 0)) (end (line 2) (character 73))))
    (element (id (node (document "d0") (qualified-name "Use::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 4)) (end (line 3) (character 16))) (parent (node (document "d0") (qualified-name "Use"))) (authored (membership (kind Import) (import (reference "A::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 11)) (end (line 3) (character 12))))))
    (element (id (node (document "d0") (qualified-name "Use::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 4)) (end (line 4) (character 16))) (parent (node (document "d0") (qualified-name "Use"))) (authored (membership (kind Import) (import (reference "B::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 11)) (end (line 4) (character 12))))))
    (element (id (node (document "d0") (qualified-name "Use::usage"))) (kind "part") (name "usage") (declared-name "usage") (range (start (line 5) (character 4)) (end (line 5) (character 23))) (parent (node (document "d0") (qualified-name "Use"))) (authored (membership (kind Feature)) (relationships (typing (reference "Thing") (range (start (line 5) (character 17)) (end (line 5) (character 22)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Use::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "A::*") (range (start (line 3) (character 11)) (end (line 3) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "A")))))
    (reference (id (source (node (document "d0") (qualified-name "Use::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "B::*") (range (start (line 4) (character 11)) (end (line 4) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "B")))))
    (reference (id (source (node (document "d0") (qualified-name "Use::usage"))) (kind featureTyping) (ordinal 0)) (authored-target "Thing") (range (start (line 5) (character 17)) (end (line 5) (character 22))) (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "A::Thing")) (node (document "d0") (qualified-name "B::Thing")))))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
