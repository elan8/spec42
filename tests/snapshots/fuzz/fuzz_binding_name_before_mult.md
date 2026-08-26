# META
~~~ini
description=Fuzz: binding connector formats name before multiplicity
type=file
~~~
# SOURCE
~~~sysml
package P {
    binding b [5] of a = c;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_binding_name_before_mult.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 1 21) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 1 25) (end 1 26))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5709f46aa2e48696b5eac15d221d5d7d2762f0382d7b912f4cb4afdfceb63a1d") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (path (named (kind package) (name "P")) (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 5) (upper 5))) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "a")) (bindTarget (reference "c")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (path (named (kind package) (name "P")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (path (named (kind package) (name "P")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "c")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/fuzz_binding_name_before_mult.md") (range (start 1 21) (end 1 22)) (probe (position 1 21))
    (reference (id (source (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (path (named (kind package) (name "P")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "a")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/fuzz_binding_name_before_mult.md") (range (start 1 25) (end 1 26)) (probe (position 1 25))
    (reference (id (source (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (path (named (kind package) (name "P")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "c")
      (outcome (status unresolved)))
    )
  )
)
~~~
