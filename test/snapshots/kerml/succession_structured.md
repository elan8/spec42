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
# FORMAT
~~~sysml
package SuccessionStructured {
    succession all [*] trigger then [*] guard;
    succession [1] entry then [*] middle;
    succession first X then Y;
    succession s first A then B;
    succession all [*] acceptable then [1] exit;
    succession x;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "144ece9b737878fb006b0daa89efe8396ffe6ebeea915096e34e21067daf206e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SuccessionStructured"))) (kind "package") (name "SuccessionStructured") (declared-name "SuccessionStructured") (range (start (line 0) (character 0)) (end (line 0) (character 252))))
    (element (id (node (document "d0") (qualified-name "SuccessionStructured::1"))) (kind "kermlDecl") (name "1") (declared-name "1") (range (start (line 2) (character 4)) (end (line 2) (character 41))) (parent (node (document "d0") (qualified-name "SuccessionStructured"))))
    (element (id (node (document "d0") (qualified-name "SuccessionStructured::all"))) (kind "kermlDecl") (name "all") (declared-name "all") (range (start (line 1) (character 4)) (end (line 1) (character 46))) (parent (node (document "d0") (qualified-name "SuccessionStructured"))))
    (element (id (node (document "d0") (qualified-name "SuccessionStructured::all#kermlDecl"))) (kind "kermlDecl") (name "all") (declared-name "all") (range (start (line 5) (character 4)) (end (line 5) (character 48))) (parent (node (document "d0") (qualified-name "SuccessionStructured"))))
    (element (id (node (document "d0") (qualified-name "SuccessionStructured::first"))) (kind "kermlDecl") (name "first") (declared-name "first") (range (start (line 3) (character 4)) (end (line 3) (character 30))) (parent (node (document "d0") (qualified-name "SuccessionStructured"))))
    (element (id (node (document "d0") (qualified-name "SuccessionStructured::s"))) (kind "kermlDecl") (name "s") (declared-name "s") (range (start (line 4) (character 4)) (end (line 4) (character 32))) (parent (node (document "d0") (qualified-name "SuccessionStructured"))))
    (element (id (node (document "d0") (qualified-name "SuccessionStructured::x"))) (kind "kermlDecl") (name "x") (declared-name "x") (range (start (line 6) (character 4)) (end (line 6) (character 17))) (parent (node (document "d0") (qualified-name "SuccessionStructured"))))
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
