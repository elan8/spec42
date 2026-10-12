# META
~~~ini
description=Effective identification keeps an absent anonymous name distinct from an unresolved first redefinition
specification=OMG KerML 1.0 (formal/2026-03-01)
clause=8.3.3.3.4
type=file
~~~
# SOURCE
~~~sysml
package Naming {
    part def Base { part existing; }
    part : Base;
    part broken : Base { part :>> missing; }
}
~~~
# EDITOR QUERIES
~~~text
probe feature_query_unresolved_names.md 2 7 features
probe feature_query_unresolved_names.md 3 25 features
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/feature_query_unresolved_names.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 1 20) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 34) (end 3 41))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:1cd0528d00aa4f73400e12c78b25ceb7c84852fa5d1a9b62964734f3589ae9f5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_query_unresolved_names.md") (path (named (kind package) (name "Naming")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base::existing"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::broken"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/feature_query_unresolved_names.md") (path (named (kind package) (name "Naming")) (named (kind part) (name "broken")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "missing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/feature_query_unresolved_names.md") (path (named (kind package) (name "Naming")) (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base")))))
    (reference (id (source (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::broken"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base")))))
    (reference (id (source (node (document "memory://snapshot/feature_query_unresolved_names.md") (path (named (kind package) (name "Naming")) (named (kind part) (name "broken")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "missing")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_query_unresolved_names.md") (path (named (kind package) (name "Naming")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_query_unresolved_names.md") (path (named (kind package) (name "Naming")) (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::broken"))) (target (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::broken"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base::existing"))) (target (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/feature_query_unresolved_names.md") (path (named (kind package) (name "Naming")) (named (kind part) (name "broken")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::broken"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/feature_query_unresolved_names.md") (path (named (kind package) (name "Naming")) (anonymous (kind part) (ordinal 0)))))
      (type (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base")) (source direct))
      (supertype (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base")))
      (subtype (node (document "memory://snapshot/feature_query_unresolved_names.md") (path (named (kind package) (name "Naming")) (anonymous (kind part) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::broken")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base::existing")))
      (featured-by (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base")))
    )
    (declaration (id (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::broken")))
      (type (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base")) (source direct))
      (supertype (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/feature_query_unresolved_names.md") (path (named (kind package) (name "Naming")) (named (kind part) (name "broken")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::broken")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/feature_query_unresolved_names.md") (range (start 2 11) (end 2 15)) (probe (position 2 11))
    (reference (id (source (node (document "memory://snapshot/feature_query_unresolved_names.md") (path (named (kind package) (name "Naming")) (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base")))))
    )
  )
  (query (document "memory://snapshot/feature_query_unresolved_names.md") (range (start 3 18) (end 3 22)) (probe (position 3 18))
    (reference (id (source (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::broken"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_query_unresolved_names.md") (qualified-name "Naming::Base")))))
    )
  )
  (query (document "memory://snapshot/feature_query_unresolved_names.md") (range (start 3 34) (end 3 41)) (probe (position 3 34))
    (reference (id (source (node (document "memory://snapshot/feature_query_unresolved_names.md") (path (named (kind package) (name "Naming")) (named (kind part) (name "broken")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "missing")
      (outcome (status unresolved)))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/feature_query_unresolved_names.md") (position 2 7)
    (target (status unresolved))
    (rename (status unresolved))
    (visible-members (candidates (member (name "Base") (qualified-name "Naming::Base") (kind "PartDefinition")) (member (name "Naming") (qualified-name "Naming") (kind "Package")) (member (name "broken") (qualified-name "Naming::broken") (kind "PartUsage")) (member (name "existing") (qualified-name "Naming::Base::existing") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (qualified-name "Naming::")
          (location (document "memory://snapshot/feature_query_unresolved_names.md") (range (start 2 4) (end 2 16)) (role Declaration))
          (declaration (range (start 2 4) (end 2 16)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Base") (target resolved))
          (typing (outcome resolved) (target "Naming::Base"))
          (effective-typing (outcome resolved) (type (qualified-name "Naming::Base") (origin direct) (provenance authored)))
          (inherited-feature (qualified-name "Naming::Base::existing") (declared-in "Naming::Base") (direct true))
          (outgoing (kind "typing") (peer "Naming::Base") (provenance authored))
        )
      )
      (referenced (status none))
    )
    (feature-queries
       (effective-identification (status resolved) (name absent) (short-name absent) (provenance Declared))
      (effective-features (status resolved)
        (feature (qualified-name "Naming::Base::existing") (authored-name "existing") (effective-identification (status resolved) (name (resolved "existing")) (short-name absent) (provenance Declared)))
      )
      (owned-features (status unsupported))
      (derived (kind DefinitionOwnedPart) (status unsupported))
      (derived (kind UsageNestedPart) (status resolved))
      (derived (kind DefinitionOwnedView) (status unsupported))
      (derived (kind UsageNestedView) (status resolved))
    )
  )
  (probe (document "memory://snapshot/feature_query_unresolved_names.md") (position 3 25)
    (target (status unresolved))
    (rename (status unresolved))
    (visible-members (candidates (member (name "Base") (qualified-name "Naming::Base") (kind "PartDefinition")) (member (name "Naming") (qualified-name "Naming") (kind "Package")) (member (name "broken") (qualified-name "Naming::broken") (kind "PartUsage")) (member (name "existing") (qualified-name "Naming::Base::existing") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (qualified-name "Naming::broken::")
          (location (document "memory://snapshot/feature_query_unresolved_names.md") (range (start 3 25) (end 3 42)) (role Declaration))
          (declaration (range (start 3 25) (end 3 42)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "redefinition") (provenance authored) (authored "missing") (target unresolved))
          (relationship (kind "typeFeaturing") (provenance implied) (target resolved))
          (redefinition (outcome unresolved))
          (effective-typing (outcome unresolved))
          (outgoing (kind "typeFeaturing") (peer "Naming::broken") (provenance implied))
        )
      )
      (referenced (status none))
    )
    (feature-queries
       (effective-identification (status resolved) (name unresolved) (short-name unresolved) (provenance FirstRedefinition))
      (effective-features (status resolved)
      )
      (owned-features (status unsupported))
      (derived (kind DefinitionOwnedPart) (status unsupported))
      (derived (kind UsageNestedPart) (status resolved))
      (derived (kind DefinitionOwnedView) (status unsupported))
      (derived (kind UsageNestedView) (status resolved))
    )
  )
  (document-symbols (document "memory://snapshot/feature_query_unresolved_names.md")
    (status resolved)
    (symbol (kind "Package") (name "Naming") (qualified-name "Naming") (location (document "memory://snapshot/feature_query_unresolved_names.md") (range (start 0 8) (end 0 14)) (role Declaration)) (declaration (range (start 0 0) (end 4 1))))
    (symbol (kind "PartDefinition") (name "Base") (qualified-name "Naming::Base") (location (document "memory://snapshot/feature_query_unresolved_names.md") (range (start 1 13) (end 1 17)) (role Declaration)) (declaration (range (start 1 4) (end 1 36))))
    (symbol (kind "PartUsage") (name "existing") (qualified-name "Naming::Base::existing") (location (document "memory://snapshot/feature_query_unresolved_names.md") (range (start 1 25) (end 1 33)) (role Declaration)) (declaration (range (start 1 20) (end 1 34))))
    (symbol (kind "PartUsage") (qualified-name "Naming::") (location (document "memory://snapshot/feature_query_unresolved_names.md") (range (start 2 4) (end 2 16)) (role Declaration)) (declaration (range (start 2 4) (end 2 16))))
    (symbol (kind "PartUsage") (name "broken") (qualified-name "Naming::broken") (location (document "memory://snapshot/feature_query_unresolved_names.md") (range (start 3 9) (end 3 15)) (role Declaration)) (declaration (range (start 3 4) (end 3 44))))
    (symbol (kind "PartUsage") (qualified-name "Naming::broken::") (location (document "memory://snapshot/feature_query_unresolved_names.md") (range (start 3 25) (end 3 42)) (role Declaration)) (declaration (range (start 3 25) (end 3 42))))
  )
)
~~~
