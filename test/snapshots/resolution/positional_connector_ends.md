# META
~~~ini
description=Authored connector ends carry their position, which their names cannot recover
type=file
~~~
# SOURCE
~~~sysml
package Ends {
    part def Pump;
    part def Tank;

    connection def Feed {
        end supply : Pump;
        end reservoir : Tank;
    }

    // The declared labels sort reservoir before supply, so a projection that recovered position
    // from a name would swap these two ends. Position is authored order and nothing else.
    connection def Reversed {
        end reservoir : Tank;
        end supply : Pump;
    }

    interface def Coupling {
        end left : Pump;
        end right : Tank;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/positional_connector_ends.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:639bf58febba197d97fa97129ac1dc01e9e1a495764a18354f4e21592889c226") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling::left"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Pump")))))
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling::right"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tank")))))
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed::reservoir"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tank")))))
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed::supply"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Pump")))))
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed::reservoir"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tank")))))
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed::supply"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Pump")))))
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling::left"))) (kind featureTyping) (ordinal 0))
      (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")))))
    (reference (id (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling::right"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")))))
    (reference (id (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed::reservoir"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")))))
    (reference (id (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed::supply"))) (kind featureTyping) (ordinal 0))
      (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")))))
    (reference (id (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed::reservoir"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")))))
    (reference (id (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed::supply"))) (kind featureTyping) (ordinal 0))
      (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling::left"))) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling::left"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling::right"))) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling::right"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed::reservoir"))) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed::reservoir"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed::supply"))) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed::supply"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed::reservoir"))) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed::reservoir"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed::supply"))) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed::supply"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling")))
      (positional-ends (authored 2) (effective 2))
    )
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling::left")))
      (featured-by (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling")))
      (type (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")) (provenance authored))
      (effective-type (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")) (source direct))
      (supertype (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling::right")))
      (featured-by (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling")))
      (type (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")) (provenance authored))
      (effective-type (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")) (source direct))
      (supertype (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed")))
      (positional-ends (authored 2) (effective 2))
    )
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed::reservoir")))
      (featured-by (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed")))
      (type (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")) (provenance authored))
      (effective-type (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")) (source direct))
      (supertype (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed::supply")))
      (featured-by (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed")))
      (type (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")) (provenance authored))
      (effective-type (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")) (source direct))
      (supertype (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")))
      (subtype (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling::left")) (scopes any))
      (subtype (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed::supply")) (scopes any))
      (subtype (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed::supply")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed")))
      (positional-ends (authored 2) (effective 2))
    )
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed::reservoir")))
      (featured-by (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed")))
      (type (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")) (provenance authored))
      (effective-type (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")) (source direct))
      (supertype (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed::supply")))
      (featured-by (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed")))
      (type (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")) (provenance authored))
      (effective-type (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")) (source direct))
      (supertype (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")))
      (subtype (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling::right")) (scopes any))
      (subtype (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed::reservoir")) (scopes any))
      (subtype (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed::reservoir")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/positional_connector_ends.md") (range (start 17 19) (end 17 23)) (probe (position 17 19))
    (reference (id (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling::left"))) (kind featureTyping) (ordinal 0) (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")))))
    )
  )
  (query (document "memory://snapshot/positional_connector_ends.md") (range (start 18 20) (end 18 24)) (probe (position 18 20))
    (reference (id (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Coupling::right"))) (kind featureTyping) (ordinal 0) (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")))))
    )
  )
  (query (document "memory://snapshot/positional_connector_ends.md") (range (start 6 24) (end 6 28)) (probe (position 6 24))
    (reference (id (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed::reservoir"))) (kind featureTyping) (ordinal 0) (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")))))
    )
  )
  (query (document "memory://snapshot/positional_connector_ends.md") (range (start 5 21) (end 5 25)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Feed::supply"))) (kind featureTyping) (ordinal 0) (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")))))
    )
  )
  (query (document "memory://snapshot/positional_connector_ends.md") (range (start 12 24) (end 12 28)) (probe (position 12 24))
    (reference (id (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed::reservoir"))) (kind featureTyping) (ordinal 0) (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Tank")))))
    )
  )
  (query (document "memory://snapshot/positional_connector_ends.md") (range (start 13 21) (end 13 25)) (probe (position 13 21))
    (reference (id (source (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Reversed::supply"))) (kind featureTyping) (ordinal 0) (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/positional_connector_ends.md") (qualified-name "Ends::Pump")))))
    )
  )
)
~~~
