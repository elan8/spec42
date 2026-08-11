# META
~~~ini
description=KerML Simple Tests: Behaviors
type=file
~~~
# SOURCE
~~~kerml
package Behaviors {
    behavior A {
        in x;
        out y = b.y1;
        composite step b : B {
            in x1 = A::x;
        }
    }
    behavior B specializes A {
        in x1;
        out var y1;
    }
    class C {
        var z = A().y;
        step a : A;
        step b : B;
        binding z = a.y;
        flow a.y to b.x1;
    }
    abstract flow msg of C;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "behaviors.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Behaviors {
    behavior A {
        in x;
        out y = b.y1;
        composite step b : B {
            in x1 = A::x;
        }
    }
    behavior B specializes A {
        in x1;
        out var y1;
    }
    class C {
        var z = A().y;
        step a : A;
        step b : B;
        binding z = a.y;
        flow a.y to b.x1;
    }
    abstract flow msg of C;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b5c9d6443f1b681e2709c68027980f11ffaa284da08b6b62635da2d377644d03") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Behaviors"))) (kind "package") (name "Behaviors") (declared-name "Behaviors"))
    (element (id (node (document "d0") (qualified-name "Behaviors::A"))) (kind "kermlDecl") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "Behaviors"))))
    (element (id (node (document "d0") (qualified-name "Behaviors::B"))) (kind "kermlDecl") (name "B") (declared-name "B") (parent (node (document "d0") (qualified-name "Behaviors"))))
    (element (id (node (document "d0") (qualified-name "Behaviors::C"))) (kind "classifier decl") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "Behaviors"))))
    (element (id (node (document "d0") (qualified-name "Behaviors::msg"))) (kind "flow") (name "msg") (declared-name "msg") (parent (node (document "d0") (qualified-name "Behaviors"))))
    (element (id (node (document "d0") (qualified-name "Behaviors::msg::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (parent (node (document "d0") (qualified-name "Behaviors::msg"))) (authored (relationships (typing (reference "C")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Behaviors::msg::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (outcome (status resolved) (target (node (document "d0") (qualified-name "Behaviors::C")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Behaviors::msg::_payload"))) (target (node (document "d0") (qualified-name "Behaviors::C"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Behaviors::msg::_payload"))) (kind featureTyping) (ordinal 0)))
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
