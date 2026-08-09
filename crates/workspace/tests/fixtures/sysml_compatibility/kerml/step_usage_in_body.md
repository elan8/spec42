# META
~~~ini
description=step_usage node: step inside a SysML definition body
type=file
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
    (element (kind "state def") (id (node (document "d0") (qualified-name "SD"))) (name "SD") (declared-name "SD"))
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
  (document "kerml/step_usage_in_body.md"
    (diagnostics
    )
  )
)
~~~
