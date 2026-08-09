# META
~~~ini
description=Wildcard import statement
type=file
~~~
# SOURCE
~~~sysml
import ScalarValues::*;
~~~
# TOKENS
~~~zig
KwImport,Ident,ColonColon,Star,Semicolon,EndOfFile,
~~~
# AST
~~~
(root
  (import_decl 'ScalarValues::*'))
~~~
# FORMAT
~~~sysml
import ScalarValues::*;
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
    (namespace_import -> 'ScalarValues'[unresolved])))
~~~
