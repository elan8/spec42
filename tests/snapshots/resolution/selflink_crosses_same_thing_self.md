# META
~~~ini
description=SelfLink crosses sameThing.self resolves to Anything::self through the cyclic end's strict ancestors
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
type=file
libraries=none
~~~
# SOURCE
~~~kerml
package P {
    abstract classifier Anything {
        feature self : Anything [1];
    }
    assoc SelfLink {
        end feature thisThing : Anything subsets sameThing crosses sameThing.self;
        end feature sameThing : Anything subsets thisThing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/selflink_crosses_same_thing_self.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/selflink_crosses_same_thing_self.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:dedc5067445e065e4eb49cf5063279d77d27304c034c1e1e25206f96213dd5d5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything::self"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")))))
    (declaration (id (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")) (subsetting (reference "thisThing")))))
    (declaration (id (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")) (subsetting (reference "sameThing")) (crossSubsetting (reference "sameThing::self")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")))))
    (reference (id (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")))))
    (reference (id (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing"))) (kind subsetting) (ordinal 0))
      (authored-target "thisThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing")))))
    (reference (id (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")))))
    (reference (id (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (kind subsetting) (ordinal 0))
      (authored-target "sameThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing")))))
    (reference (id (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "sameThing::self")
      (outcome (status resolved) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything::self")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything::self"))) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing"))) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing"))) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (kind subsetting) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything::self"))) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing"))) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")))
      (subtype (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything::self")) (scopes any))
      (subtype (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing")) (scopes any))
      (subtype (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything::self")))
      (featured-by (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")))
      (type (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")) (provenance authored))
      (effective-type (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")) (source direct))
      (supertype (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")) (scopes any))
      (subtype (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing"))) (cyclic true)
      (featured-by (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink")))
      (type (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")) (provenance authored))
      (effective-type (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")) (source direct))
      (effective-type (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")) (source inherited) (from (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything::self"))))
      (effective-type (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")) (source inherited) (from (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))))
      (effective-type (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")) (source inherited) (from (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing"))))
      (supertype (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything::self")) (scopes any feature))
      (supertype (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing")) (scopes any feature))
      (subtype (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (cyclic true)
      (featured-by (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink")))
      (type (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")) (provenance authored))
      (effective-type (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")) (source direct))
      (effective-type (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")) (source inherited) (from (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything::self"))))
      (effective-type (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")) (source inherited) (from (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))))
      (effective-type (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")) (source inherited) (from (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing"))))
      (supertype (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything::self")) (scopes any feature))
      (supertype (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing")) (scopes any feature))
      (subtype (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/selflink_crosses_same_thing_self.md") (range (start 2 23) (end 2 31)) (probe (position 2 23))
    (reference (id (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything::self"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")))))
    )
  )
  (query (document "memory://snapshot/selflink_crosses_same_thing_self.md") (range (start 6 32) (end 6 40)) (probe (position 6 32))
    (reference (id (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")))))
    )
  )
  (query (document "memory://snapshot/selflink_crosses_same_thing_self.md") (range (start 6 49) (end 6 58)) (probe (position 6 49))
    (reference (id (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing"))) (kind subsetting) (ordinal 0) (authored-target "thisThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing")))))
    )
  )
  (query (document "memory://snapshot/selflink_crosses_same_thing_self.md") (range (start 5 32) (end 5 40)) (probe (position 5 32))
    (reference (id (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything")))))
    )
  )
  (query (document "memory://snapshot/selflink_crosses_same_thing_self.md") (range (start 5 49) (end 5 58)) (probe (position 5 49))
    (reference (id (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (kind subsetting) (ordinal 0) (authored-target "sameThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::sameThing")))))
    )
  )
  (query (document "memory://snapshot/selflink_crosses_same_thing_self.md") (range (start 5 67) (end 5 81)) (probe (position 5 67))
    (reference (id (source (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::SelfLink::thisThing"))) (kind crossSubsetting) (ordinal 0) (authored-target "sameThing::self")
      (outcome (status resolved) (target (node (document "memory://snapshot/selflink_crosses_same_thing_self.md") (qualified-name "P::Anything::self")))))
    )
  )
)
~~~
