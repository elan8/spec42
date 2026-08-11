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
  (document "inherited_diamond_dedup.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c614115f4ae5ec96fcf33741aa8bf337b8ec1a1f7d216c98f539b0894599d2e6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Diamond"))) (kind "package") (name "Diamond") (declared-name "Diamond"))
    (element (id (node (document "d0") (qualified-name "Diamond::Base"))) (kind "part def") (name "Base") (declared-name "Base") (parent (node (document "d0") (qualified-name "Diamond"))))
    (element (id (node (document "d0") (qualified-name "Diamond::Base::Member"))) (kind "part def") (name "Member") (declared-name "Member") (parent (node (document "d0") (qualified-name "Diamond::Base"))))
    (element (id (node (document "d0") (qualified-name "Diamond::Diamond"))) (kind "part def") (name "Diamond") (declared-name "Diamond") (parent (node (document "d0") (qualified-name "Diamond"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Left")) (specializes (reference "Right")))))
    (element (id (node (document "d0") (qualified-name "Diamond::Diamond::p"))) (kind "part") (name "p") (declared-name "p") (parent (node (document "d0") (qualified-name "Diamond::Diamond"))) (authored (membership (kind Feature)) (relationships (typing (reference "Member")))))
    (element (id (node (document "d0") (qualified-name "Diamond::Left"))) (kind "part def") (name "Left") (declared-name "Left") (parent (node (document "d0") (qualified-name "Diamond"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Base")))))
    (element (id (node (document "d0") (qualified-name "Diamond::Right"))) (kind "part def") (name "Right") (declared-name "Right") (parent (node (document "d0") (qualified-name "Diamond"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Diamond::Diamond"))) (kind specialization) (ordinal 0)) (authored-target "Left") (outcome (status resolved) (target (node (document "d0") (qualified-name "Diamond::Left")))))
    (reference (id (source (node (document "d0") (qualified-name "Diamond::Diamond"))) (kind specialization) (ordinal 1)) (authored-target "Right") (outcome (status resolved) (target (node (document "d0") (qualified-name "Diamond::Right")))))
    (reference (id (source (node (document "d0") (qualified-name "Diamond::Diamond::p"))) (kind featureTyping) (ordinal 0)) (authored-target "Member") (outcome (status resolved) (target (node (document "d0") (qualified-name "Diamond::Base::Member")))))
    (reference (id (source (node (document "d0") (qualified-name "Diamond::Left"))) (kind specialization) (ordinal 0)) (authored-target "Base") (outcome (status resolved) (target (node (document "d0") (qualified-name "Diamond::Base")))))
    (reference (id (source (node (document "d0") (qualified-name "Diamond::Right"))) (kind specialization) (ordinal 0)) (authored-target "Base") (outcome (status resolved) (target (node (document "d0") (qualified-name "Diamond::Base")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Diamond::Diamond"))) (target (node (document "d0") (qualified-name "Diamond::Left"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Diamond::Diamond"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Diamond::Diamond"))) (target (node (document "d0") (qualified-name "Diamond::Right"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Diamond::Diamond"))) (kind specialization) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Diamond::Diamond::p"))) (target (node (document "d0") (qualified-name "Diamond::Base::Member"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Diamond::Diamond::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Diamond::Left"))) (target (node (document "d0") (qualified-name "Diamond::Base"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Diamond::Left"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Diamond::Right"))) (target (node (document "d0") (qualified-name "Diamond::Base"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Diamond::Right"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 21) (end 4 25)) (probe (position 4 21))
      (reference
        (source (document "d0") (qualified-name "Diamond::Left"))
        (kind specialization) (ordinal 0) (authored-target "Base")
        (range (start 4 21) (end 4 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Diamond::Base") (range (start 1 4) (end 1 50)))
        )
      )
    )
    (query (range (start 5 22) (end 5 26)) (probe (position 5 22))
      (reference
        (source (document "d0") (qualified-name "Diamond::Right"))
        (kind specialization) (ordinal 0) (authored-target "Base")
        (range (start 5 22) (end 5 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Diamond::Base") (range (start 1 4) (end 1 50)))
        )
      )
    )
    (query (range (start 6 24) (end 6 28)) (probe (position 6 24))
      (reference
        (source (document "d0") (qualified-name "Diamond::Diamond"))
        (kind specialization) (ordinal 0) (authored-target "Left")
        (range (start 6 24) (end 6 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Diamond::Left") (range (start 4 4) (end 4 26)))
        )
      )
    )
    (query (range (start 6 30) (end 6 35)) (probe (position 6 30))
      (reference
        (source (document "d0") (qualified-name "Diamond::Diamond"))
        (kind specialization) (ordinal 1) (authored-target "Right")
        (range (start 6 30) (end 6 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Diamond::Right") (range (start 5 4) (end 5 27)))
        )
      )
    )
    (query (range (start 7 17) (end 7 23)) (probe (position 7 17))
      (reference
        (source (document "d0") (qualified-name "Diamond::Diamond::p"))
        (kind featureTyping) (ordinal 0) (authored-target "Member")
        (range (start 7 17) (end 7 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Diamond::Base::Member") (range (start 2 8) (end 2 24)))
        )
      )
    )
  )
)
~~~
