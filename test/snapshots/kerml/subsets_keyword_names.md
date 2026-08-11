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
# FORMAT
~~~sysml
step s1 subsets step;
feature f1 redefines step;
feature f2 subsets do, step;

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1d16f421f7c93925788f5d8f054dfc93102e56af4b29710a99272be879043e11") (contract-version "canonical-resolution-v1"))
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
