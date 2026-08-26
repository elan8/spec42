# META
~~~ini
description=Cyclic alias bindings report non_converged_resolution per reference while the publication stays complete
type=file
~~~
# SOURCE
~~~sysml
package P {
    part def Target;
    alias A for B;
    alias B for A;
    part uses : A;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/cyclic_alias_non_convergence.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 2 16) (end 2 17))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 3 16) (end 3 17))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8daf4c591c114554419cd04fef02b44bb7e071afff1d8f48c66a9a5f0962231b") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::A"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "B")))))
    (declaration (id (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::B"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "A")))))
    (declaration (id (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::Target"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::uses"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::A"))) (kind aliasBinding) (ordinal 0))
      (authored-target "B")
      (outcome (status nonConverged)))
    (reference (id (source (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::B"))) (kind aliasBinding) (ordinal 0))
      (authored-target "A")
      (outcome (status nonConverged)))
    (reference (id (source (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::uses"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::A")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::uses"))) (target (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::uses"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::A")))
      (subtype (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::uses")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::uses")))
      (type (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::A")) (source direct))
      (supertype (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::A")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/cyclic_alias_non_convergence.md") (range (start 2 16) (end 2 17)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::A"))) (kind aliasBinding) (ordinal 0) (authored-target "B")
      (outcome (status nonConverged)))
    )
  )
  (query (document "memory://snapshot/cyclic_alias_non_convergence.md") (range (start 3 16) (end 3 17)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::B"))) (kind aliasBinding) (ordinal 0) (authored-target "A")
      (outcome (status nonConverged)))
    )
  )
  (query (document "memory://snapshot/cyclic_alias_non_convergence.md") (range (start 4 16) (end 4 17)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::uses"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/cyclic_alias_non_convergence.md") (qualified-name "P::A")))))
    )
  )
)
~~~
