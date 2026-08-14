# META
~~~ini
description=KerML binding connector with named form + multiplicity + 'of' disambiguation
type=file
~~~
# SOURCE
~~~kerml
package BindingNamedMult {
    binding instant[instantNum] of startShot = endShot;
    binding all startShot = endShot;
    binding x bind a = b;
    binding [0..1] a = b;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/binding_named_mult.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 1 35) (end 1 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 1 47) (end 1 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 16) (end 2 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 28) (end 2 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 19) (end 3 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 23) (end 3 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 19) (end 4 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 23) (end 4 24))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:646e291c0e43444b0e1d47c6e4b0376ce33307d9f638fd9a8f7e21ed5ec55562") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/binding_named_mult.md") (qualified-name "BindingNamedMult"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (facts (multiplicity (lower expression) (upper expression))) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "startShot")) (bindTarget (reference "endShot"))))
    (declaration (id (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 1))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "startShot")) (bindTarget (reference "endShot"))))
    (declaration (id (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 2))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "a")) (bindTarget (reference "b"))))
    (declaration (id (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 3))))) (kind bind) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "a")) (bindTarget (reference "b"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 1))))) (kind bindSource) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 2))))) (kind bindSource) (ordinal 0))
      (authored-target "a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 3))))) (kind bindSource) (ordinal 0))
      (authored-target "a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "endShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 1))))) (kind bindTarget) (ordinal 0))
      (authored-target "endShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 2))))) (kind bindTarget) (ordinal 0))
      (authored-target "b")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 3))))) (kind bindTarget) (ordinal 0))
      (authored-target "b")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/binding_named_mult.md") (range (start 1 35) (end 1 44)) (probe (position 1 35))
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/binding_named_mult.md") (range (start 2 16) (end 2 25)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 1))))) (kind bindSource) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/binding_named_mult.md") (range (start 3 19) (end 3 20)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 2))))) (kind bindSource) (ordinal 0) (authored-target "a")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/binding_named_mult.md") (range (start 4 19) (end 4 20)) (probe (position 4 19))
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 3))))) (kind bindSource) (ordinal 0) (authored-target "a")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/binding_named_mult.md") (range (start 1 47) (end 1 54)) (probe (position 1 47))
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "endShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/binding_named_mult.md") (range (start 2 28) (end 2 35)) (probe (position 2 28))
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 1))))) (kind bindTarget) (ordinal 0) (authored-target "endShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/binding_named_mult.md") (range (start 3 23) (end 3 24)) (probe (position 3 23))
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 2))))) (kind bindTarget) (ordinal 0) (authored-target "b")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/binding_named_mult.md") (range (start 4 23) (end 4 24)) (probe (position 4 23))
    (reference (id (source (node (document "memory://snapshot/binding_named_mult.md") (anonymous (kind bind) (ordinal 3))))) (kind bindTarget) (ordinal 0) (authored-target "b")
      (outcome (status unresolved)))
  )
)
~~~
