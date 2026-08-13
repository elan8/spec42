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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1 4) (end 1 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 2 4) (end 2 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 3 4) (end 3 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 4 4) (end 4 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:646e291c0e43444b0e1d47c6e4b0376ce33307d9f638fd9a8f7e21ed5ec55562") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/binding_named_mult.md") (qualified-name "BindingNamedMult"))) (kind package) (membership (kind owning) (visibility default)))
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
