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
  (document "fuzz_binding_name_before_mult.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 1 4) (end 1 28))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "7dac02e943abcf3ade3c210395140d238b2a14682cb19602ff8ab3ba2e6c099a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P"))
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
