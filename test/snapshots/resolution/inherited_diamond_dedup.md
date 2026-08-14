# META
~~~ini
description=Inherited diamond specialization target is deduplicated
type=file
~~~
# SOURCE
~~~sysml
package Diamond {
    part def Base {
        part def Member;
    }
    part def Left :> Base;
    part def Right :> Base;
    part def Diamond :> Left, Right {
        part p : Member;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/inherited_diamond_dedup.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5f640a66de8a0500cf8788e675fd95d17030c1a22737fd8b731bc97257da2470") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base::Member"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Left")) (specialization (reference "Right")))))
    (declaration (id (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond::p"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Member")))))
    (declaration (id (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Left"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Right"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond"))) (kind specialization) (ordinal 0))
      (authored-target "Left")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Left")))))
    (reference (id (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond"))) (kind specialization) (ordinal 1))
      (authored-target "Right")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Right")))))
    (reference (id (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "Member")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base::Member")))))
    (reference (id (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Left"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base")))))
    (reference (id (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Right"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond"))) (target (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Left"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond"))) (target (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Right"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond"))) (kind specialization) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond::p"))) (target (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base::Member"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Left"))) (target (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Left"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Right"))) (target (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Right"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base")))
      (subtype (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Left")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Right")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base::Member")))
      (subtype (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond::p")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond")))
      (supertype (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Left")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Right")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond::p")))
      (type (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base::Member")) (provenance authored))
      (effective-type (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base::Member")) (source direct))
      (supertype (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base::Member")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Left")))
      (supertype (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Right")))
      (supertype (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/inherited_diamond_dedup.md") (range (start 6 24) (end 6 28)) (probe (position 6 24))
    (reference (id (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond"))) (kind specialization) (ordinal 0) (authored-target "Left")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Left")))))
    )
  )
  (query (document "memory://snapshot/inherited_diamond_dedup.md") (range (start 6 30) (end 6 35)) (probe (position 6 30))
    (reference (id (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond"))) (kind specialization) (ordinal 1) (authored-target "Right")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Right")))))
    )
  )
  (query (document "memory://snapshot/inherited_diamond_dedup.md") (range (start 7 17) (end 7 23)) (probe (position 7 17))
    (reference (id (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Diamond::p"))) (kind featureTyping) (ordinal 0) (authored-target "Member")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base::Member")))))
    )
  )
  (query (document "memory://snapshot/inherited_diamond_dedup.md") (range (start 4 21) (end 4 25)) (probe (position 4 21))
    (reference (id (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Left"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base")))))
    )
  )
  (query (document "memory://snapshot/inherited_diamond_dedup.md") (range (start 5 22) (end 5 26)) (probe (position 5 22))
    (reference (id (source (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Right"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_diamond_dedup.md") (qualified-name "Diamond::Base")))))
    )
  )
)
~~~
