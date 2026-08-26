# META
~~~ini
description=Duplicate qualified names retain source-document identities
type=multi
~~~
# SOURCE
## alpha.sysml
~~~sysml
package P {
    part def Engine;
}
~~~
## beta.sysml
~~~sysml
package P {
    part def Engine;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/alpha.sysml"
    (diagnostics
    )
  )
  (document "memory://snapshot/beta.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:6bda00768fa43bd1246923d0312e382648b4511fd4f5adc87630a43fa6ba944a") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/alpha.sysml") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alpha.sysml") (qualified-name "P::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/beta.sysml") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/beta.sysml") (qualified-name "P::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
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
