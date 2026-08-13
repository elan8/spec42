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
  (document "memory://snapshot/ambiguous_import_candidates.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 5 17) (end 5 22))
        (related-information
          (related
            (uri "memory://snapshot/ambiguous_import_candidates.md")
            (range (start 0 12) (end 0 27))
          )
          (related
            (uri "memory://snapshot/ambiguous_import_candidates.md")
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ea187c4c9af2de2666b58cc933e4f449d756cefe98736d706a4f065a7afc57bb") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "A"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "A::Thing"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "B"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "B::Thing"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "Use"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "A") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "B") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "Use::usage"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/ambiguous_import_candidates.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "A")))))
    (reference (id (source (node (document "memory://snapshot/ambiguous_import_candidates.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "B")))))
    (reference (id (source (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "Use::usage"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "A::Thing")) (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "B::Thing")))))
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
  (query (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 3 11) (end 3 15)) (probe (position 3 11))
    (reference (id (source (node (document "memory://snapshot/ambiguous_import_candidates.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "A")))))
  )
  (query (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 4 11) (end 4 15)) (probe (position 4 11))
    (reference (id (source (node (document "memory://snapshot/ambiguous_import_candidates.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "B")))))
  )
  (query (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 5 17) (end 5 22)) (probe (position 5 17))
    (reference (id (source (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "Use::usage"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "A::Thing")) (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "B::Thing")))))
  )
)
~~~
