# META
~~~ini
description=Positional ends a connection-like declaration inherits, kept apart from those it authors
type=file
~~~
# SOURCE
~~~sysml
package Ends {
    part def Pump;
    part def Tank;

    // Authors both ends itself.
    connection def Feed {
        end supply : Pump;
        end reservoir : Tank;
    }

    // Authors none. Its ends are entirely inherited, so authored and effective differ.
    connection def InheritedPair :> Feed;

    // Authors one, which redefines the first inherited end positionally rather than adding a
    // third. The effective count stays two.
    connection def PartialOverride :> Feed {
        end supply : Pump;
    }

    // Two hops away: the count is read off the transitive closure, not one level of it.
    connection def Grandchild :> InheritedPair;

    // Authors one end and inherits nothing.
    connection def Lone {
        end only : Pump;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/effective_positional_ends.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "incomplete_connection_like_end_pair")
        (source "semantic")
        (range (start 23 4) (end 25 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:0901f26312779071a67b3b90aec10cb735e504347908bcc635f5757b73311a68") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::reservoir"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tank")))))
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::supply"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Pump")))))
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Grandchild"))) (kind connection-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "InheritedPair")))))
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::InheritedPair"))) (kind connection-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Feed")))))
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Lone"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Lone::only"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Pump")))))
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride"))) (kind connection-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Feed")))))
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride::supply"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Pump")))))
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Tank"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::reservoir"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Tank")))))
    (reference (id (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::supply"))) (kind featureTyping) (ordinal 0))
      (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")))))
    (reference (id (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Grandchild"))) (kind specialization) (ordinal 0))
      (authored-target "InheritedPair")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::InheritedPair")))))
    (reference (id (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::InheritedPair"))) (kind specialization) (ordinal 0))
      (authored-target "Feed")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed")))))
    (reference (id (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Lone::only"))) (kind featureTyping) (ordinal 0))
      (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")))))
    (reference (id (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride"))) (kind specialization) (ordinal 0))
      (authored-target "Feed")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed")))))
    (reference (id (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride::supply"))) (kind featureTyping) (ordinal 0))
      (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::reservoir"))) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Tank"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::reservoir"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::supply"))) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::supply"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Grandchild"))) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::InheritedPair"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Grandchild"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::InheritedPair"))) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::InheritedPair"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Lone::only"))) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Lone::only"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride"))) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride::supply"))) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride::supply"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::reservoir"))) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::supply"))) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Lone::only"))) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Lone"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride::supply"))) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::supply"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride::supply"))) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed")))
      (positional-ends (authored 2) (effective 2))
      (subtype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::InheritedPair")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::reservoir")))
      (featured-by (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed")))
      (type (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Tank")) (provenance authored))
      (effective-type (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Tank")) (source direct))
      (supertype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Tank")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::supply")))
      (featured-by (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed")))
      (type (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")) (provenance authored))
      (effective-type (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")) (source direct))
      (supertype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")) (scopes any))
      (subtype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride::supply")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Grandchild")))
      (positional-ends (authored 0) (effective 2))
      (supertype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::InheritedPair")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::InheritedPair")))
      (positional-ends (authored 0) (effective 2))
      (supertype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Grandchild")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Lone")))
      (positional-ends (authored 1) (effective 1))
    )
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Lone::only")))
      (featured-by (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Lone")))
      (type (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")) (provenance authored))
      (effective-type (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")) (source direct))
      (supertype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride")))
      (positional-ends (authored 1) (effective 2))
      (supertype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride::supply")))
      (featured-by (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride")))
      (type (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")) (provenance authored))
      (effective-type (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")) (source direct))
      (effective-type (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")) (source inherited) (from (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::supply"))))
      (supertype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::supply")) (scopes any feature))
      (supertype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")))
      (subtype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::supply")) (scopes any))
      (subtype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Lone::only")) (scopes any))
      (subtype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride::supply")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Tank")))
      (subtype (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::reservoir")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/effective_positional_ends.md") (range (start 7 24) (end 7 28)) (probe (position 7 24))
    (reference (id (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::reservoir"))) (kind featureTyping) (ordinal 0) (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Tank")))))
    )
  )
  (query (document "memory://snapshot/effective_positional_ends.md") (range (start 6 21) (end 6 25)) (probe (position 6 21))
    (reference (id (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed::supply"))) (kind featureTyping) (ordinal 0) (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")))))
    )
  )
  (query (document "memory://snapshot/effective_positional_ends.md") (range (start 20 33) (end 20 46)) (probe (position 20 33))
    (reference (id (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Grandchild"))) (kind specialization) (ordinal 0) (authored-target "InheritedPair")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::InheritedPair")))))
    )
  )
  (query (document "memory://snapshot/effective_positional_ends.md") (range (start 11 36) (end 11 40)) (probe (position 11 36))
    (reference (id (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::InheritedPair"))) (kind specialization) (ordinal 0) (authored-target "Feed")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed")))))
    )
  )
  (query (document "memory://snapshot/effective_positional_ends.md") (range (start 24 19) (end 24 23)) (probe (position 24 19))
    (reference (id (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Lone::only"))) (kind featureTyping) (ordinal 0) (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")))))
    )
  )
  (query (document "memory://snapshot/effective_positional_ends.md") (range (start 15 38) (end 15 42)) (probe (position 15 38))
    (reference (id (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride"))) (kind specialization) (ordinal 0) (authored-target "Feed")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Feed")))))
    )
  )
  (query (document "memory://snapshot/effective_positional_ends.md") (range (start 16 21) (end 16 25)) (probe (position 16 21))
    (reference (id (source (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::PartialOverride::supply"))) (kind featureTyping) (ordinal 0) (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_positional_ends.md") (qualified-name "Ends::Pump")))))
    )
  )
)
~~~
