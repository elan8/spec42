# META
~~~ini
description=Navigation edit queries publish references rename collisions and visible imported inherited members
type=file
~~~
# SOURCE
~~~sysml
package Types {
    part def Base {
        attribute inheritedMember;
    }
    part def Other;
}
package Use {
    import Types::*;
    part def Child :> Base {
        attribute ownMember;
        part x : Other;
    }
}
~~~
# EDITOR QUERIES
~~~ini
probe editor_queries.md 10 17 rename=Base
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/editor_queries.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:67e8dc0acb0a3208d0be8842b8c3fcec049fe083a338fd0fbe918f4672d0aaed") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Base::inheritedMember"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Other"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "Types") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base"))))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child::ownMember"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child::x"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Other"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/editor_queries.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Types")
      (outcome (status resolved) (target (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types")))))
    (reference (id (source (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Base")))))
    (reference (id (source (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Other")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child"))) (target (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child::x"))) (target (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Other"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child::x"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/editor_queries.md") (range (start 7 11) (end 7 19)) (probe (position 7 11))
    (reference (id (source (node (document "memory://snapshot/editor_queries.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Types")
      (outcome (status resolved) (target (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types")))))
  )
  (query (document "memory://snapshot/editor_queries.md") (range (start 8 22) (end 8 26)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Base")))))
  )
  (query (document "memory://snapshot/editor_queries.md") (range (start 10 17) (end 10 22)) (probe (position 10 17))
    (reference (id (source (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child::x"))) (kind featureTyping) (ordinal 0) (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Other")))))
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/editor_queries.md") (position 10 17)
    (target (status resolved) (candidate (name "Other") (location (document "memory://snapshot/editor_queries.md") (range (start 4 13) (end 4 18)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/editor_queries.md") (range (start 4 13) (end 4 18)) (role Declaration)) (location (document "memory://snapshot/editor_queries.md") (range (start 10 17) (end 10 22)) (role Reference))))
    (rename (status collision) (candidates (candidate (name "Base") (location (document "memory://snapshot/editor_queries.md") (range (start 1 13) (end 1 17)) (role Declaration)))))
    (visible-members (candidates (member (name "Base") (qualified-name "Types::Base") (kind "PartDefinition")) (member (name "Child") (qualified-name "Use::Child") (kind "PartDefinition")) (member (name "Other") (qualified-name "Types::Other") (kind "PartDefinition")) (member (name "Types") (qualified-name "Types") (kind "Package")) (member (name "Use") (qualified-name "Use") (kind "Package")) (member (name "inheritedMember") (qualified-name "Types::Base::inheritedMember") (kind "AttributeUsage")) (member (name "ownMember") (qualified-name "Use::Child::ownMember") (kind "AttributeUsage")) (member (name "x") (qualified-name "Use::Child::x") (kind "PartUsage"))))
  )
)
~~~
