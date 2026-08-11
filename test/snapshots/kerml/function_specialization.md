# META
~~~ini
description=Function and predicate definitions with specialization clauses
type=file
~~~
# SOURCE
~~~kerml
package FuncSpec {
    function F specializes Base::G { }
    function H :> Base::I, Base::J { }
    abstract function K :> Base::L;
    predicate P specializes Base::Q { }
    predicate R :> Base::S;
    function FI specializes Base::G intersects Base::H { }
    predicate PI specializes Base::Q intersects Base::R, Base::S { }
    inv I { not x }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "function_specialization.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package FuncSpec {
    function F specializes Base::G { }
    function H :> Base::I, Base::J { }
    abstract function K :> Base::L;
    predicate P specializes Base::Q { }
    predicate R :> Base::S;
    function FI specializes Base::G intersects Base::H { }
    predicate PI specializes Base::Q intersects Base::R, Base::S { }
    inv I { not x }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "acd43b8e528f1be93e2512b4afaf0b5a4447d0e1f0cd45c5931d0741a293f352") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "FuncSpec"))) (kind "package") (name "FuncSpec") (declared-name "FuncSpec") (range (start (line 0) (character 0)) (end (line 0) (character 350))))
    (element (id (node (document "d0") (qualified-name "FuncSpec::F"))) (kind "kermlDecl") (name "F") (declared-name "F") (range (start (line 1) (character 4)) (end (line 1) (character 38))) (parent (node (document "d0") (qualified-name "FuncSpec"))))
    (element (id (node (document "d0") (qualified-name "FuncSpec::FI"))) (kind "kermlDecl") (name "FI") (declared-name "FI") (range (start (line 6) (character 4)) (end (line 6) (character 58))) (parent (node (document "d0") (qualified-name "FuncSpec"))))
    (element (id (node (document "d0") (qualified-name "FuncSpec::H"))) (kind "kermlDecl") (name "H") (declared-name "H") (range (start (line 2) (character 4)) (end (line 2) (character 38))) (parent (node (document "d0") (qualified-name "FuncSpec"))))
    (element (id (node (document "d0") (qualified-name "FuncSpec::I"))) (kind "kermlDecl") (name "I") (declared-name "I") (range (start (line 8) (character 4)) (end (line 8) (character 19))) (parent (node (document "d0") (qualified-name "FuncSpec"))))
    (element (id (node (document "d0") (qualified-name "FuncSpec::K"))) (kind "kermlDecl") (name "K") (declared-name "K") (range (start (line 3) (character 4)) (end (line 3) (character 35))) (parent (node (document "d0") (qualified-name "FuncSpec"))))
    (element (id (node (document "d0") (qualified-name "FuncSpec::P"))) (kind "kermlDecl") (name "P") (declared-name "P") (range (start (line 4) (character 4)) (end (line 4) (character 39))) (parent (node (document "d0") (qualified-name "FuncSpec"))))
    (element (id (node (document "d0") (qualified-name "FuncSpec::PI"))) (kind "kermlDecl") (name "PI") (declared-name "PI") (range (start (line 7) (character 4)) (end (line 7) (character 68))) (parent (node (document "d0") (qualified-name "FuncSpec"))))
    (element (id (node (document "d0") (qualified-name "FuncSpec::R"))) (kind "kermlDecl") (name "R") (declared-name "R") (range (start (line 5) (character 4)) (end (line 5) (character 27))) (parent (node (document "d0") (qualified-name "FuncSpec"))))
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
