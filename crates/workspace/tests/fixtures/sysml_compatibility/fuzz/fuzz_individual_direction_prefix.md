# META
~~~ini
description=Fuzz: individual usage with direction prefix preserves 'individual' keyword
type=file
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
(model
  (namespace
    (occurrence_usage in individual 'it')))
~~~
