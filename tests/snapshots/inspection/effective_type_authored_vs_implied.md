# META
~~~ini
description=An effective type inherited through an authored subsetting or redefinition keeps `provenance authored`; one inherited only through an implied same-name redefinition is `provenance implied`, so compact editor surfaces can drop the implied library closure
type=file
~~~
# SOURCE
~~~sysml
package EffectiveTypeProvenance {
    part def Wheel;

    part def Base {
        part contactPatch : Wheel;
    }

    // `contactPatch` here has no authored typing; it is an implied same-name
    // redefinition of `Base::contactPatch`, so it inherits `Wheel` implicitly.
    part def ImpliedDerived :> Base {
        part contactPatch;
    }

    // `road` authors the redefinition, so it inherits `Wheel` through something
    // the author wrote.
    part def AuthoredDerived :> Base {
        part road :>> contactPatch;
    }
}
~~~
# EDITOR QUERIES
~~~text
probe effective_type_authored_vs_implied.md 10 13 hover
probe effective_type_authored_vs_implied.md 16 13 hover
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/effective_type_authored_vs_implied.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:0c8ad3fad9d15cd6833c104d0f52d7f73ddd8ab5561b795b125bb549fc686d3e"))
  (declarations
    (declaration (id (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived::road"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "contactPatch")))))
    (declaration (id (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::ImpliedDerived"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::ImpliedDerived::contactPatch"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base")))))
    (reference (id (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived::road"))) (kind redefinition) (ordinal 0))
      (authored-target "contactPatch")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch")))))
    (reference (id (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::ImpliedDerived"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived"))) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived::road"))) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived::road"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch"))) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::ImpliedDerived"))) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::ImpliedDerived"))) (kind specialization) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived::road"))) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch"))) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::ImpliedDerived::contactPatch"))) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::ImpliedDerived::contactPatch"))) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::ImpliedDerived"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived")))
      (supertype (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived::road")))
      (featured-by (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived")))
      (effective-type (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Wheel")) (source inherited) (from (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch"))))
      (supertype (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch")) (scopes any feature))
      (supertype (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Wheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base")))
      (subtype (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::ImpliedDerived")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch")))
      (featured-by (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base")))
      (type (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Wheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Wheel")) (source direct))
      (supertype (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Wheel")) (scopes any))
      (subtype (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived::road")) (scopes any feature))
      (subtype (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::ImpliedDerived::contactPatch")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::ImpliedDerived")))
      (supertype (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::ImpliedDerived::contactPatch")))
      (featured-by (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::ImpliedDerived")))
      (effective-type (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Wheel")) (source inherited) (from (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch"))))
      (supertype (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch")) (scopes any feature))
      (supertype (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Wheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Wheel")))
      (subtype (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 15 32) (end 15 36)) (probe (position 15 32))
    (reference (id (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base")))))
    )
  )
  (query (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 16 22) (end 16 34)) (probe (position 16 22))
    (reference (id (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::AuthoredDerived::road"))) (kind redefinition) (ordinal 0) (authored-target "contactPatch")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch")))))
    )
  )
  (query (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 4 28) (end 4 33)) (probe (position 4 28))
    (reference (id (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base::contactPatch"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Wheel")))))
    )
  )
  (query (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 9 31) (end 9 35)) (probe (position 9 31))
    (reference (id (source (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::ImpliedDerived"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_type_authored_vs_implied.md") (qualified-name "EffectiveTypeProvenance::Base")))))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/effective_type_authored_vs_implied.md") (position 10 13)
    (target (status resolved) (candidate (name "contactPatch") (location (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 10 13) (end 10 25)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 10 13) (end 10 25)) (role Declaration))))
    (rename (status ready) (name "contactPatch") (range (start 10 13) (end 10 25)) (occurrences 1))
    (visible-members (candidates (member (name "AuthoredDerived") (qualified-name "EffectiveTypeProvenance::AuthoredDerived") (kind "PartDefinition")) (member (name "Base") (qualified-name "EffectiveTypeProvenance::Base") (kind "PartDefinition")) (member (name "EffectiveTypeProvenance") (qualified-name "EffectiveTypeProvenance") (kind "Package")) (member (name "ImpliedDerived") (qualified-name "EffectiveTypeProvenance::ImpliedDerived") (kind "PartDefinition")) (member (name "Wheel") (qualified-name "EffectiveTypeProvenance::Wheel") (kind "PartDefinition")) (member (name "contactPatch") (qualified-name "EffectiveTypeProvenance::Base::contactPatch") (kind "PartUsage")) (member (name "contactPatch") (qualified-name "EffectiveTypeProvenance::ImpliedDerived::contactPatch") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "contactPatch")
          (qualified-name "EffectiveTypeProvenance::ImpliedDerived::contactPatch")
          (location (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 10 13) (end 10 25)) (role Declaration))
          (declaration (range (start 10 8) (end 10 26)))
          (membership (kind feature) (visibility private) (provenance default))
          (relationship (kind "typeFeaturing") (provenance implied) (target resolved))
          (relationship (kind "redefinition") (provenance implied) (target resolved))
          (outgoing (kind "redefinition") (peer "EffectiveTypeProvenance::Base::contactPatch") (provenance implied))
          (outgoing (kind "typeFeaturing") (peer "EffectiveTypeProvenance::ImpliedDerived") (provenance implied))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/effective_type_authored_vs_implied.md") (position 16 13)
    (target (status resolved) (candidate (name "road") (location (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 16 13) (end 16 17)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 16 13) (end 16 17)) (role Declaration))))
    (rename (status ready) (name "road") (range (start 16 13) (end 16 17)) (occurrences 1))
    (visible-members (candidates (member (name "AuthoredDerived") (qualified-name "EffectiveTypeProvenance::AuthoredDerived") (kind "PartDefinition")) (member (name "Base") (qualified-name "EffectiveTypeProvenance::Base") (kind "PartDefinition")) (member (name "EffectiveTypeProvenance") (qualified-name "EffectiveTypeProvenance") (kind "Package")) (member (name "ImpliedDerived") (qualified-name "EffectiveTypeProvenance::ImpliedDerived") (kind "PartDefinition")) (member (name "Wheel") (qualified-name "EffectiveTypeProvenance::Wheel") (kind "PartDefinition")) (member (name "contactPatch") (qualified-name "EffectiveTypeProvenance::Base::contactPatch") (kind "PartUsage")) (member (name "road") (qualified-name "EffectiveTypeProvenance::AuthoredDerived::road") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "road")
          (qualified-name "EffectiveTypeProvenance::AuthoredDerived::road")
          (location (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 16 13) (end 16 17)) (role Declaration))
          (declaration (range (start 16 8) (end 16 35)))
          (membership (kind feature) (visibility private) (provenance default))
          (relationship (kind "redefinition") (provenance authored) (authored "contactPatch") (target resolved))
          (relationship (kind "typeFeaturing") (provenance implied) (target resolved))
          (redefinition (outcome resolved) (target "EffectiveTypeProvenance::Base::contactPatch"))
          (effective-typing (outcome resolved) (type (qualified-name "EffectiveTypeProvenance::Wheel") (origin inherited) (provenance authored)))
          (outgoing (kind "redefinition") (peer "EffectiveTypeProvenance::Base::contactPatch") (provenance authored))
          (outgoing (kind "typeFeaturing") (peer "EffectiveTypeProvenance::AuthoredDerived") (provenance implied))
        )
      )
      (referenced (status none))
    )
  )
  (document-symbols (document "memory://snapshot/effective_type_authored_vs_implied.md")
    (status resolved)
    (symbol (kind "Package") (name "EffectiveTypeProvenance") (qualified-name "EffectiveTypeProvenance") (location (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 0 8) (end 0 31)) (role Declaration)) (declaration (range (start 0 0) (end 18 1))))
    (symbol (kind "PartDefinition") (name "Wheel") (qualified-name "EffectiveTypeProvenance::Wheel") (location (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 1 13) (end 1 18)) (role Declaration)) (declaration (range (start 1 4) (end 1 19))))
    (symbol (kind "PartDefinition") (name "Base") (qualified-name "EffectiveTypeProvenance::Base") (location (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 3 13) (end 3 17)) (role Declaration)) (declaration (range (start 3 4) (end 5 5))))
    (symbol (kind "PartUsage") (name "contactPatch") (qualified-name "EffectiveTypeProvenance::Base::contactPatch") (location (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 4 13) (end 4 25)) (role Declaration)) (declaration (range (start 4 8) (end 4 34))))
    (symbol (kind "PartDefinition") (name "ImpliedDerived") (qualified-name "EffectiveTypeProvenance::ImpliedDerived") (location (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 9 13) (end 9 27)) (role Declaration)) (declaration (range (start 9 4) (end 11 5))))
    (symbol (kind "PartUsage") (name "contactPatch") (qualified-name "EffectiveTypeProvenance::ImpliedDerived::contactPatch") (location (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 10 13) (end 10 25)) (role Declaration)) (declaration (range (start 10 8) (end 10 26))))
    (symbol (kind "PartDefinition") (name "AuthoredDerived") (qualified-name "EffectiveTypeProvenance::AuthoredDerived") (location (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 15 13) (end 15 28)) (role Declaration)) (declaration (range (start 15 4) (end 17 5))))
    (symbol (kind "PartUsage") (name "road") (qualified-name "EffectiveTypeProvenance::AuthoredDerived::road") (location (document "memory://snapshot/effective_type_authored_vs_implied.md") (range (start 16 13) (end 16 17)) (role Declaration)) (declaration (range (start 16 8) (end 16 35))))
  )
)
~~~
# HOVER RESULTS
~~~sexpr
(hover-reports
  (probe (document "memory://snapshot/effective_type_authored_vs_implied.md") (position 10 13) (status available)
    (hover
      (identity (kind "part") (name "contactPatch") (direct-types))
      (qualified-name "EffectiveTypeProvenance::ImpliedDerived::contactPatch")
      (destination (labels "contactPatch" "EffectiveTypeProvenance::ImpliedDerived::contactPatch") (uri "memory://snapshot/effective_type_authored_vs_implied.md") (position 10 13))
    )
  )
  (probe (document "memory://snapshot/effective_type_authored_vs_implied.md") (position 16 13) (status available)
    (hover
      (identity (kind "part") (name "road") (direct-types))
      (qualified-name "EffectiveTypeProvenance::AuthoredDerived::road")
      (inherited-type (type "EffectiveTypeProvenance::Wheel") (from "EffectiveTypeProvenance::Base::contactPatch"))
      (destination (labels "road" "EffectiveTypeProvenance::AuthoredDerived::road") (uri "memory://snapshot/effective_type_authored_vs_implied.md") (position 16 13))
      (destination (labels "EffectiveTypeProvenance::Wheel") (uri "memory://snapshot/effective_type_authored_vs_implied.md") (position 1 13))
      (destination (labels "EffectiveTypeProvenance::Base::contactPatch") (uri "memory://snapshot/effective_type_authored_vs_implied.md") (position 4 13))
    )
  )
)
~~~
# HOVER MARKDOWN
## effective_type_authored_vs_implied.md:10:13
~~~markdown
`part` **[contactPatch](memory://snapshot/effective_type_authored_vs_implied.md#L11)**

`EffectiveTypeProvenance::ImpliedDerived::contactPatch`
~~~
## effective_type_authored_vs_implied.md:16:13
~~~markdown
`part` **[road](memory://snapshot/effective_type_authored_vs_implied.md#L17)**

`EffectiveTypeProvenance::AuthoredDerived::road`

Inherited type [`EffectiveTypeProvenance::Wheel`](memory://snapshot/effective_type_authored_vs_implied.md#L2) from [`EffectiveTypeProvenance::Base::contactPatch`](memory://snapshot/effective_type_authored_vs_implied.md#L5)
~~~
