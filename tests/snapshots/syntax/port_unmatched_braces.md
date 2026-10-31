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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:3a3f529c3b880ababb842cc7d474740439d7cd44596c2ffdd0ca14e5905925ca"))
  (declarations
    (declaration (id (node (document "memory://snapshot/port_unmatched_braces.md") (qualified-name "Ports"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/port_unmatched_braces.md") (qualified-name "Ports::Power"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/port_unmatched_braces.md") (path (named (kind package) (name "Ports")) (named (kind port-def) (name "Power")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Power") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/port_unmatched_braces.md") (qualified-name "Ports::Power::nested"))) (kind port) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind conjugation) (source (node (document "memory://snapshot/port_unmatched_braces.md") (path (named (kind package) (name "Ports")) (named (kind port-def) (name "Power")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/port_unmatched_braces.md") (qualified-name "Ports::Power"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/port_unmatched_braces.md") (qualified-name "Ports::Power::nested"))) (target (node (document "memory://snapshot/port_unmatched_braces.md") (qualified-name "Ports::Power"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/port_unmatched_braces.md") (qualified-name "Ports::Power::nested")))
      (featured-by (node (document "memory://snapshot/port_unmatched_braces.md") (qualified-name "Ports::Power")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
