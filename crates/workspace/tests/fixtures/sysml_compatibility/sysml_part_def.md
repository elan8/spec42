# META
~~~ini
description=SysML part definition
type=file
~~~
# SOURCE
~~~sysml
part def Vehicle { }
~~~
# TOKENS
~~~zig
KwPart,KwDef,Ident,OpenCurly,CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (part_def 'Vehicle'))
~~~
# FORMAT
~~~sysml
part def Vehicle { }
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
(model
  (namespace
    (part_def 'Vehicle')))
~~~
