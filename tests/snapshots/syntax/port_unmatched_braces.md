# META
~~~ini
description=A port definition with an unmatched brace is not DefinitionBody and must recover
type=file
~~~
# SOURCE
~~~sysml
package Ports {
    port def Power {
        port nested;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/port_unmatched_braces.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_closing_brace")
        (source "parser")
        (range (start 2 20) (end 2 20))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:2649cd441ef3a52f30bd6a381eb8b167498e4e2cdedfb418497d66bb5131b861"))
  (declarations
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
