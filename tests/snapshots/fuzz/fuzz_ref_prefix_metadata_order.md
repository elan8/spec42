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
        (code "recovered_calc_body_element")
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:0cf98a2d7bcc836b0936f19263a6a8b20cf11b33b70102e9d58f7e1a40f5db3d"))
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
