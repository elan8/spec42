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
(model
  (namespace
    (state_def 'SD'
      (step_usage 's')
      (step_usage 's2' :> 'step'[unresolved]))))
~~~
