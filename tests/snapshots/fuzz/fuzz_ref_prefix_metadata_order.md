# META
~~~ini
description=Fuzz: ref keyword precedes prefix metadata annotations for correct reparse
type=file
~~~
# SOURCE
~~~sysml
package P {
    class C {
        ref #MyAnnotation self : C;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_ref_prefix_metadata_order.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 2 8) (end 3 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:d85418246b5dca4cbe258c358576c276e2d695ae806ba31c21f0675fc38f8e95") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_ref_prefix_metadata_order.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_ref_prefix_metadata_order.md") (qualified-name "P::C"))) (kind class-def) (membership (kind owning) (visibility default)))
  )
  (references
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
)
~~~
