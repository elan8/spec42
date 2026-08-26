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
# EDITOR QUERIES
~~~ini
probe ambiguous_import_candidates.md 5 18
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ea187c4c9af2de2666b58cc933e4f449d756cefe98736d706a4f065a7afc57bb"))
  (declarations
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "A"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "A::Thing"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "B"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "B::Thing"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "Use"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (path (named (kind package) (name "Use")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "A") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (path (named (kind package) (name "Use")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "B") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "Use::usage"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/ambiguous_import_candidates.md") (path (named (kind package) (name "Use")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "A")))))
    (reference (id (source (node (document "memory://snapshot/ambiguous_import_candidates.md") (path (named (kind package) (name "Use")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
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
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 3 11) (end 3 15)) (probe (position 3 11))
    (reference (id (source (node (document "memory://snapshot/ambiguous_import_candidates.md") (path (named (kind package) (name "Use")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "A")))))
    )
  )
  (query (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 4 11) (end 4 15)) (probe (position 4 11))
    (reference (id (source (node (document "memory://snapshot/ambiguous_import_candidates.md") (path (named (kind package) (name "Use")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "B")))))
    )
  )
  (query (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 5 17) (end 5 22)) (probe (position 5 17))
    (reference (id (source (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "Use::usage"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "A::Thing")) (node (document "memory://snapshot/ambiguous_import_candidates.md") (qualified-name "B::Thing")))))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/ambiguous_import_candidates.md") (position 5 18)
    (target (status ambiguous) (candidates (candidate (name "Thing") (location (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 0 21) (end 0 26)) (role Declaration))) (candidate (name "Thing") (location (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 1 21) (end 1 26)) (role Declaration)))))
    (rename (status ambiguous) (candidates 2))
    (visible-members (candidates (member (name "A") (qualified-name "A") (kind "Package")) (member (name "B") (qualified-name "B") (kind "Package")) (member (name "Thing") (qualified-name "A::Thing") (kind "PartDefinition")) (member (name "Thing") (qualified-name "B::Thing") (kind "PartDefinition")) (member (name "Use") (qualified-name "Use") (kind "Package")) (member (name "usage") (qualified-name "Use::usage") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "usage")
          (qualified-name "Use::usage")
          (location (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 5 9) (end 5 14)) (role Declaration))
          (declaration (range (start 5 4) (end 5 23)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Thing") (target ambiguous 2))
          (typing (outcome ambiguous) (candidate "A::Thing") (candidate "B::Thing"))
          (effective-typing (outcome ambiguous))
        )
      )
      (reference-kind featureTyping)
      (referenced (status ambiguous)
        (element (kind "PartDefinition")
          (name "Thing")
          (qualified-name "A::Thing")
          (location (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 0 21) (end 0 26)) (role Declaration))
          (declaration (range (start 0 12) (end 0 27)))
          (membership (kind owning) (visibility public) (provenance default))
        )
        (element (kind "PartDefinition")
          (name "Thing")
          (qualified-name "B::Thing")
          (location (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 1 21) (end 1 26)) (role Declaration))
          (declaration (range (start 1 12) (end 1 27)))
          (membership (kind owning) (visibility public) (provenance default))
        )
      )
    )
  )
  (document-symbols (document "memory://snapshot/ambiguous_import_candidates.md")
    (status resolved)
    (symbol (kind "Package") (name "A") (qualified-name "A") (location (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 0 8) (end 0 9)) (role Declaration)) (declaration (range (start 0 0) (end 0 29))))
    (symbol (kind "PartDefinition") (name "Thing") (qualified-name "A::Thing") (location (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 0 21) (end 0 26)) (role Declaration)) (declaration (range (start 0 12) (end 0 27))))
    (symbol (kind "Package") (name "B") (qualified-name "B") (location (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 1 8) (end 1 9)) (role Declaration)) (declaration (range (start 1 0) (end 1 29))))
    (symbol (kind "PartDefinition") (name "Thing") (qualified-name "B::Thing") (location (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 1 21) (end 1 26)) (role Declaration)) (declaration (range (start 1 12) (end 1 27))))
    (symbol (kind "Package") (name "Use") (qualified-name "Use") (location (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 2 8) (end 2 11)) (role Declaration)) (declaration (range (start 2 0) (end 6 1))))
    (symbol (kind "Import") (qualified-name "Use::") (location (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 3 4) (end 3 16)) (role Declaration)) (declaration (range (start 3 4) (end 3 16))))
    (symbol (kind "Import") (qualified-name "Use::") (location (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 4 4) (end 4 16)) (role Declaration)) (declaration (range (start 4 4) (end 4 16))))
    (symbol (kind "PartUsage") (name "usage") (qualified-name "Use::usage") (location (document "memory://snapshot/ambiguous_import_candidates.md") (range (start 5 9) (end 5 14)) (role Declaration)) (declaration (range (start 5 4) (end 5 23))))
  )
)
~~~
