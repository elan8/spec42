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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:bf995632649d7cba41dca292a084af1250a870738627e15a24bf822dda281b7a"))
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
