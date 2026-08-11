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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "succession_structured.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "144ece9b737878fb006b0daa89efe8396ffe6ebeea915096e34e21067daf206e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SuccessionStructured"))) (kind "package") (name "SuccessionStructured") (declared-name "SuccessionStructured"))
    (element (id (node (document "d0") (qualified-name "SuccessionStructured::1"))) (kind "kermlDecl") (name "1") (declared-name "1") (parent (node (document "d0") (qualified-name "SuccessionStructured"))))
    (element (id (node (document "d0") (qualified-name "SuccessionStructured::all"))) (kind "kermlDecl") (name "all") (declared-name "all") (parent (node (document "d0") (qualified-name "SuccessionStructured"))))
    (element (id (node (document "d0") (qualified-name "SuccessionStructured::all#kermlDecl"))) (kind "kermlDecl") (name "all") (declared-name "all") (parent (node (document "d0") (qualified-name "SuccessionStructured"))))
    (element (id (node (document "d0") (qualified-name "SuccessionStructured::first"))) (kind "kermlDecl") (name "first") (declared-name "first") (parent (node (document "d0") (qualified-name "SuccessionStructured"))))
    (element (id (node (document "d0") (qualified-name "SuccessionStructured::s"))) (kind "kermlDecl") (name "s") (declared-name "s") (parent (node (document "d0") (qualified-name "SuccessionStructured"))))
    (element (id (node (document "d0") (qualified-name "SuccessionStructured::x"))) (kind "kermlDecl") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "SuccessionStructured"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
