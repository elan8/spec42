# META
~~~ini
description=KerML 7.3.2.3 allows specialization cycles as shared extent; only an entirely closed cycle that omits Anything is an error
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
type=file
libraries=none
~~~
# SOURCE
~~~kerml
package Cycles {
    abstract classifier Anything;

    // Kernel SelfLink idiom: mutual subsetting asserts the ends are the same, and each end
    // also specialises outward (feature typing). Not an entirely closed cycle.
    assoc SelfLink {
        end feature thisThing : Anything subsets sameThing;
        end feature sameThing : Anything subsets thisThing;
    }

    // Mutual specialisation that also specialises Anything is shared extent, not an error.
    classifier A specializes B;
    classifier B specializes A, Anything;

    // Entirely closed feature cycle that never reaches Anything.
    feature x :> z;
    feature y :> x;
    feature z :> y;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/specialization_cycle_equivalence.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "specialization_cycle")
        (source "semantic")
        (range (start 15 4) (end 15 19))
      )
      (diagnostic
        (severity error)
        (code "specialization_cycle")
        (source "semantic")
        (range (start 16 4) (end 16 19))
      )
      (diagnostic
        (severity error)
        (code "specialization_cycle")
        (source "semantic")
        (range (start 17 4) (end 17 19))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/specialization_cycle_equivalence.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "specialization_cycle")
        (source "semantic")
        (range (start 15 4) (end 15 19))
      )
      (diagnostic
        (severity error)
        (code "specialization_cycle")
        (source "semantic")
        (range (start 16 4) (end 16 19))
      )
      (diagnostic
        (severity error)
        (code "specialization_cycle")
        (source "semantic")
        (range (start 17 4) (end 17 19))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e03d9c46f5aca8564d34806eda7486fff4de97a2710fc31673df86b28e1c5c76"))
  (declarations
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "B")))))
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A")) (specialization (reference "Anything")))))
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")) (subsetting (reference "thisThing")))))
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")) (subsetting (reference "sameThing")))))
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "z")))))
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "x")))))
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::z"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "y")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::A"))) (kind specialization) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B")))))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::A")))))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B"))) (kind specialization) (ordinal 1))
      (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")))))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")))))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing"))) (kind subsetting) (ordinal 0))
      (authored-target "thisThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing")))))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")))))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing"))) (kind subsetting) (ordinal 0))
      (authored-target "sameThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing")))))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::x"))) (kind subsetting) (ordinal 0))
      (authored-target "z")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::z")))))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::y"))) (kind subsetting) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::x")))))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::z"))) (kind subsetting) (ordinal 0))
      (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::y")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::A"))) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::A"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B"))) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B"))) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B"))) (kind specialization) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing"))) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing"))) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing"))) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing"))) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::x"))) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::z"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::x"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::y"))) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::y"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::z"))) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::z"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing"))) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing"))) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::A"))) (cyclic true)
      (supertype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")))
      (subtype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing")) (scopes any))
      (subtype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B"))) (cyclic true)
      (supertype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::A")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::A")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing"))) (cyclic true)
      (featured-by (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink")))
      (type (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")) (provenance authored))
      (effective-type (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")) (source direct))
      (effective-type (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")) (source inherited) (from (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing"))))
      (effective-type (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")) (source inherited) (from (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing"))))
      (supertype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing")) (scopes any feature))
      (subtype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing"))) (cyclic true)
      (featured-by (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink")))
      (type (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")) (provenance authored))
      (effective-type (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")) (source direct))
      (effective-type (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")) (source inherited) (from (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing"))))
      (effective-type (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")) (source inherited) (from (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing"))))
      (supertype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing")) (scopes any feature))
      (subtype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::x"))) (cyclic true)
      (supertype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::y")) (scopes any feature))
      (supertype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::z")) (scopes any feature))
      (subtype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::y")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::y"))) (cyclic true)
      (supertype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::x")) (scopes any feature))
      (supertype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::z")) (scopes any feature))
      (subtype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::z")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::z"))) (cyclic true)
      (supertype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::x")) (scopes any feature))
      (supertype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::y")) (scopes any feature))
      (subtype (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::x")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/specialization_cycle_equivalence.md") (range (start 11 29) (end 11 30)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::A"))) (kind specialization) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B")))))
    )
  )
  (query (document "memory://snapshot/specialization_cycle_equivalence.md") (range (start 12 29) (end 12 30)) (probe (position 12 29))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::A")))))
    )
  )
  (query (document "memory://snapshot/specialization_cycle_equivalence.md") (range (start 12 32) (end 12 40)) (probe (position 12 32))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::B"))) (kind specialization) (ordinal 1) (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")))))
    )
  )
  (query (document "memory://snapshot/specialization_cycle_equivalence.md") (range (start 7 32) (end 7 40)) (probe (position 7 32))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")))))
    )
  )
  (query (document "memory://snapshot/specialization_cycle_equivalence.md") (range (start 7 49) (end 7 58)) (probe (position 7 49))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing"))) (kind subsetting) (ordinal 0) (authored-target "thisThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing")))))
    )
  )
  (query (document "memory://snapshot/specialization_cycle_equivalence.md") (range (start 6 32) (end 6 40)) (probe (position 6 32))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::Anything")))))
    )
  )
  (query (document "memory://snapshot/specialization_cycle_equivalence.md") (range (start 6 49) (end 6 58)) (probe (position 6 49))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::thisThing"))) (kind subsetting) (ordinal 0) (authored-target "sameThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::SelfLink::sameThing")))))
    )
  )
  (query (document "memory://snapshot/specialization_cycle_equivalence.md") (range (start 15 17) (end 15 18)) (probe (position 15 17))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::x"))) (kind subsetting) (ordinal 0) (authored-target "z")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::z")))))
    )
  )
  (query (document "memory://snapshot/specialization_cycle_equivalence.md") (range (start 16 17) (end 16 18)) (probe (position 16 17))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::y"))) (kind subsetting) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::x")))))
    )
  )
  (query (document "memory://snapshot/specialization_cycle_equivalence.md") (range (start 17 17) (end 17 18)) (probe (position 17 17))
    (reference (id (source (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::z"))) (kind subsetting) (ordinal 0) (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/specialization_cycle_equivalence.md") (qualified-name "Cycles::y")))))
    )
  )
)
~~~
