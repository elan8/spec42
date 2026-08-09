# META
~~~ini
description=KerML succession with structured parsing (stdlib patterns from StatePerformances/TransitionPerformances)
type=file
~~~
# SOURCE
~~~kerml
package SuccessionStructured {
    succession all [*] trigger then [*] guard;
    succession [1] entry then [*] middle;
    succession first X then Y;
    succession s first A then B;
    succession all [*] acceptable then [1] exit;
    succession x;
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwSuccession,KwAll,OpenSquare,Star,CloseSquare,Ident,KwThen,OpenSquare,Star,CloseSquare,Ident,Semicolon,
KwSuccession,OpenSquare,DecimalValue,CloseSquare,KwEntry,KwThen,OpenSquare,Star,CloseSquare,Ident,Semicolon,
KwSuccession,KwFirst,Ident,KwThen,Ident,Semicolon,
KwSuccession,Ident,KwFirst,Ident,KwThen,Ident,Semicolon,
KwSuccession,KwAll,OpenSquare,Star,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,KwExit,Semicolon,
KwSuccession,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'SuccessionStructured'
    (succession_def multiplicity
      (connector_end)
      (connector_end))
    (succession_def multiplicity
      (connector_end)
      (connector_end))
    (succession_as_usage
      (connector_end)
      (connector_end))
    (succession_def 's'
      (connector_end)
      (connector_end))
    (succession_def multiplicity
      (connector_end)
      (connector_end))
    (succession_def 'x')))
~~~
# FORMAT
~~~sysml
package SuccessionStructured {
    succession all [*] trigger then [*] guard;
    succession [1] entry then [*] middle;
    first X then Y;
    succession s first A then B;
    succession all [*] acceptable then [1] exit;
    succession x;
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'SuccessionStructured'
      (succession_def
        (multiplicity_range [*])
        (connector_end 'trigger')
        (connector_end 'guard'))
      (succession_def
        (multiplicity_range [1])
        (connector_end 'entry')
        (connector_end 'middle'))
      (succession_def
        (connector_end 'X')
        (connector_end 'Y'))
      (succession_def 's'
        (connector_end 'A')
        (connector_end 'B'))
      (succession_def
        (multiplicity_range [*])
        (connector_end 'acceptable')
        (connector_end 'exit'))
      (succession_def 'x'))))
~~~
