# META
~~~ini
description=Permissive QN parsing: keywords used as specialization target names
type=file
semantic_graph=skip
semantic_graph_skip_reason=standalone KerML step and feature declarations with keyword names are opaque parser fallback nodes; subsetting targets are unavailable as structured semantic inputs
~~~
# SOURCE
~~~kerml
step s1 subsets step;
feature f1 redefines step;
feature f2 subsets do, step;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "subsets_keyword_names.md"
    (diagnostics
    )
  )
)
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
# FORMAT
~~~sysml
step s1 subsets step;
feature f1 redefines step;
feature f2 subsets do, step;

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "25b4fb7754582a135721818142d5794223c6f8718c01479243e74337bfbc5b8c") (contract-version "canonical-resolution-v1"))
  (structure
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
