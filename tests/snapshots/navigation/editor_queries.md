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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:67e8dc0acb0a3208d0be8842b8c3fcec049fe083a338fd0fbe918f4672d0aaed") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Base::inheritedMember"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Other"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (path (named (kind package) (name "Use")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "Types") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child::ownMember"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child::x"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Other")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/editor_queries.md") (path (named (kind package) (name "Use")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
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
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Base::inheritedMember"))) (target (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child::ownMember"))) (target (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child::x"))) (target (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Base")))
      (subtype (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Base::inheritedMember")))
      (featured-by (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Base")))
    )
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Other")))
      (subtype (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child::x")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child")))
      (supertype (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child::ownMember")))
      (featured-by (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child")))
    )
    (declaration (id (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child::x")))
      (featured-by (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child")))
      (type (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Other")) (provenance authored))
      (effective-type (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Other")) (source direct))
      (supertype (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Other")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/editor_queries.md") (range (start 7 11) (end 7 19)) (probe (position 7 11))
    (reference (id (source (node (document "memory://snapshot/editor_queries.md") (path (named (kind package) (name "Use")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Types")
      (outcome (status resolved) (target (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types")))))
    )
  )
  (query (document "memory://snapshot/editor_queries.md") (range (start 8 22) (end 8 26)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Base")))))
    )
  )
  (query (document "memory://snapshot/editor_queries.md") (range (start 10 17) (end 10 22)) (probe (position 10 17))
    (reference (id (source (node (document "memory://snapshot/editor_queries.md") (qualified-name "Use::Child::x"))) (kind featureTyping) (ordinal 0) (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/editor_queries.md") (qualified-name "Types::Other")))))
    )
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
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "x")
          (qualified-name "Use::Child::x")
          (location (document "memory://snapshot/editor_queries.md") (range (start 10 13) (end 10 14)) (role Declaration))
          (declaration (range (start 10 8) (end 10 23)))
          (membership (kind feature) (visibility private) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Other") (target resolved))
          (relationship (kind "typeFeaturing") (provenance implied) (target resolved))
          (typing (outcome resolved) (target "Types::Other"))
          (effective-typing (outcome resolved) (type (qualified-name "Types::Other") (origin direct)))
          (outgoing (kind "typeFeaturing") (peer "Use::Child") (provenance implied))
          (outgoing (kind "typing") (peer "Types::Other") (provenance authored))
        )
      )
      (reference-kind featureTyping)
      (referenced (status resolved)
        (element (kind "PartDefinition")
          (name "Other")
          (qualified-name "Types::Other")
          (location (document "memory://snapshot/editor_queries.md") (range (start 4 13) (end 4 18)) (role Declaration))
          (declaration (range (start 4 4) (end 4 19)))
          (membership (kind owning) (visibility public) (provenance default))
          (incoming (kind "typing") (peer "Use::Child::x") (provenance authored))
        )
      )
    )
  )
  (document-symbols (document "memory://snapshot/editor_queries.md")
    (status resolved)
    (symbol (kind "Package") (name "Types") (qualified-name "Types") (location (document "memory://snapshot/editor_queries.md") (range (start 0 8) (end 0 13)) (role Declaration)) (declaration (range (start 0 0) (end 5 1))))
    (symbol (kind "PartDefinition") (name "Base") (qualified-name "Types::Base") (location (document "memory://snapshot/editor_queries.md") (range (start 1 13) (end 1 17)) (role Declaration)) (declaration (range (start 1 4) (end 3 5))))
    (symbol (kind "AttributeUsage") (name "inheritedMember") (qualified-name "Types::Base::inheritedMember") (location (document "memory://snapshot/editor_queries.md") (range (start 2 18) (end 2 33)) (role Declaration)) (declaration (range (start 2 8) (end 2 34))))
    (symbol (kind "PartDefinition") (name "Other") (qualified-name "Types::Other") (location (document "memory://snapshot/editor_queries.md") (range (start 4 13) (end 4 18)) (role Declaration)) (declaration (range (start 4 4) (end 4 19))))
    (symbol (kind "Package") (name "Use") (qualified-name "Use") (location (document "memory://snapshot/editor_queries.md") (range (start 6 8) (end 6 11)) (role Declaration)) (declaration (range (start 6 0) (end 12 1))))
    (symbol (kind "Import") (qualified-name "Use::") (location (document "memory://snapshot/editor_queries.md") (range (start 7 4) (end 7 20)) (role Declaration)) (declaration (range (start 7 4) (end 7 20))))
    (symbol (kind "PartDefinition") (name "Child") (qualified-name "Use::Child") (location (document "memory://snapshot/editor_queries.md") (range (start 8 13) (end 8 18)) (role Declaration)) (declaration (range (start 8 4) (end 11 5))))
    (symbol (kind "AttributeUsage") (name "ownMember") (qualified-name "Use::Child::ownMember") (location (document "memory://snapshot/editor_queries.md") (range (start 9 18) (end 9 27)) (role Declaration)) (declaration (range (start 9 8) (end 9 28))))
    (symbol (kind "PartUsage") (name "x") (qualified-name "Use::Child::x") (location (document "memory://snapshot/editor_queries.md") (range (start 10 13) (end 10 14)) (role Declaration)) (declaration (range (start 10 8) (end 10 23))))
  )
)
~~~
