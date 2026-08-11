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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "step_usage_in_body.md"
    (diagnostics
    )
  )
)
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
# EXPECTED
~~~
semantic.unresolved_name 'step'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'step'
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e9e2624754eb01275138bbffc80303620ebea34703648a8c0852b73a2ef83245") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SD"))) (kind "state def") (name "SD") (declared-name "SD") (range (start (line 0) (character 0)) (end (line 0) (character 54))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
