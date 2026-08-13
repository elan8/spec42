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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1 4) (end 1 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:5709f46aa2e48696b5eac15d221d5d7d2762f0382d7b912f4cb4afdfceb63a1d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_binding_name_before_mult.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
  )
  (references
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
)
~~~
