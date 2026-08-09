# META
~~~ini
description=step_usage node: step inside a SysML definition body
type=file
semantic_graph=skip
semantic_graph_skip_reason=strictly parsed non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~kerml
state def SD {
    step s;
    step s2 subsets step;
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'step'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'step'
~~~
# TOKENS
~~~zig
KwState,KwDef,Ident,OpenCurly,
KwStep,Ident,Semicolon,
KwStep,Ident,KwSubsets,KwStep,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (state_def 'SD'
    (step_usage)
    (step_usage)))
~~~
# FORMAT
~~~sysml
state def SD {
    step s;
    step s2 subsets step;
}

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
