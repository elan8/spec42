# META
~~~ini
description=KerML Binding Connector: bind keyword and per-end multiplicities
type=file
~~~
# SOURCE
~~~kerml
package P {
    class C {
        feature x;
        feature y;
        feature startShot;
        feature endShot;
        feature baseEdges;

        binding [1] bind [0..*] x = [0..*] y;
        binding b bind lhs = rhs;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/binding_connector_bind_kw.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1 4) (end 10 5))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 2 8) (end 3 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 3 8) (end 4 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 4 8) (end 5 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 5 8) (end 6 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 6 8) (end 8 8))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:934a6cefbf30848d75428ac15e460810e90dd4066ce9e38abcbdd0d519da4e8d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/binding_connector_bind_kw.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
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
