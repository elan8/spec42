# META
~~~ini
description=Fuzz: individual usage with direction prefix preserves 'individual' keyword
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
in individual it;
~~~
# TOKENS
~~~zig
KwIn,KwIndividual,Ident,Semicolon,EndOfFile,
~~~
# AST
~~~
(root
  (individual_usage in individual 'it'))
~~~
# FORMAT
~~~sysml
in individual it;

~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz/fuzz_individual_direction_prefix.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "sysml")
        (range (start 0 0) (end 0 17))
      )
    )
  )
)
~~~
