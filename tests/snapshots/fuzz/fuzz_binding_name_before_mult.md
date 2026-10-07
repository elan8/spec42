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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:07e56f401e3c90b26346a222523947d1bcbe988d438d076ab81aeb240800d63b"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (path (named (kind package) (name "P")) (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 5) (upper 5))) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "a")) (bindTarget (reference "c")))))
    (declaration (id (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (path (named (kind package) (name "P")) (anonymous (kind bind) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (path (named (kind package) (name "P")) (anonymous (kind bind) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (path (named (kind package) (name "P")) (anonymous (kind bind) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (path (named (kind package) (name "P")) (anonymous (kind bind) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
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
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (path (named (kind package) (name "P")) (anonymous (kind bind) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (path (named (kind package) (name "P")) (anonymous (kind bind) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (path (named (kind package) (name "P")) (anonymous (kind bind) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (path (named (kind package) (name "P")) (anonymous (kind bind) (ordinal 0)) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
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
