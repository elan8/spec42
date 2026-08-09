# META
~~~ini
description=Permissive QN parsing: keywords used as specialization target names
type=file
semantic_graph=skip
semantic_graph_skip_reason=strictly parsed non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~kerml
step s1 subsets step;
feature f1 redefines step;
feature f2 subsets do, step;
~~~
# EXPECTED
~~~
semantic.unresolved_name 'step'
semantic.unresolved_name 'step'
semantic.unresolved_name 'do'
semantic.unresolved_name 'step'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'step'
semantic.unresolved_name 'step'
semantic.unresolved_name 'do'
semantic.unresolved_name 'step'
~~~
# TOKENS
~~~zig
KwStep,Ident,KwSubsets,KwStep,Semicolon,
KwFeature,Ident,KwRedefines,KwStep,Semicolon,
KwFeature,Ident,KwSubsets,KwDo,Comma,KwStep,Semicolon,EndOfFile,
~~~
# AST
~~~
(root
  (step_def)
  (feature_def 'f1' :>> 'step')
  (feature_def 'f2' :> 'do', 'step'))
~~~
# FORMAT
~~~sysml
step s1 subsets step;
feature f1 redefines step;
feature f2 subsets do, step;

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
