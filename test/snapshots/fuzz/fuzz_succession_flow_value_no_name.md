# META
~~~ini
description=Fuzz: succession flow with value expression but no name preserves value in formatting
type=file
~~~
# SOURCE
~~~sysml
package P {
    class Container {
        step a1 : Action1;
        succession a1 then a2;
        flow a1.y to a2.x;
        succession flow=sf from a1.y to a2.x;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_succession_flow_value_no_name.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 2 8) (end 3 8))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 3 8) (end 4 8))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 4 8) (end 5 8))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 5 8) (end 6 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:d48ca10a6f48f735e0a0bb7d958924cd9a3d725bd7fff02b464f78b88fd73630") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container"))) (kind class-def) (membership (kind owning) (visibility default)))
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
