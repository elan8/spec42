# META
~~~ini
description=Coverage: KerML behavior, step, function, expression, predicate, bool, inv, interaction, flow, succession flow
type=file
~~~
# SOURCE
~~~kerml
package BehaviorCoverage {
    behavior Action1 {
        in x;
        out y;
    }

    step s1 : Action1;

    function F {
        in a;
        return feature result : Integer;
    }

    expr E { in x; 1 + x }

    predicate P { in x : Boolean; x }

    bool b { true }

    inv I { true }
    inv false NegI { false }

    interaction Inter {
        in x;
        out y;
    }

    class Container {
        step a1 : Action1;
        step a2 : Action1;
        succession a1 then a2;
        flow a1.y to a2.x;
        succession flow sf from a1.y to a2.x;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_behaviors.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package BehaviorCoverage {
    behavior Action1 {
        in x;
        out y;
    }

    step s1 : Action1;

    function F {
        in a;
        return feature result : Integer;
    }

    expr E { in x; 1 + x }

    predicate P { in x : Boolean; x }

    bool b { true }

    inv I { true }
    inv false NegI { false }

    interaction Inter {
        in x;
        out y;
    }

    class Container {
        step a1 : Action1;
        step a2 : Action1;
        succession a1 then a2;
        flow a1.y to a2.x;
        succession flow sf from a1.y to a2.x;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "0800067208f80370ef39de1ddbd4172ef8136a533e5661a771216e306654888c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage"))) (kind "package") (name "BehaviorCoverage") (declared-name "BehaviorCoverage"))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::Action1"))) (kind "kermlDecl") (name "Action1") (declared-name "Action1") (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::Container"))) (kind "classifier decl") (name "Container") (declared-name "Container") (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::E"))) (kind "kermlDecl") (name "E") (declared-name "E") (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::F"))) (kind "kermlDecl") (name "F") (declared-name "F") (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::I"))) (kind "kermlDecl") (name "I") (declared-name "I") (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::Inter"))) (kind "kermlDecl") (name "Inter") (declared-name "Inter") (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::P"))) (kind "kermlDecl") (name "P") (declared-name "P") (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::b"))) (kind "kermlDecl") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::false"))) (kind "kermlDecl") (name "false") (declared-name "false") (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::s1"))) (kind "kermlDecl") (name "s1") (declared-name "s1") (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
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
