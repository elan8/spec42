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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "SuccessionStructured"))) (name "SuccessionStructured") (declared-name "SuccessionStructured")
      (contains
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SuccessionStructured::1"))) (name "1") (declared-name "1"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SuccessionStructured::all"))) (name "all") (declared-name "all"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SuccessionStructured::all#kermlDecl"))) (name "all") (declared-name "all"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SuccessionStructured::first"))) (name "first") (declared-name "first"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SuccessionStructured::s"))) (name "s") (declared-name "s"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SuccessionStructured::x"))) (name "x") (declared-name "x"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
