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
    (element (id (node (document "d0") (qualified-name "Behaviors"))) (kind "package") (name "Behaviors") (declared-name "Behaviors") (range (start (line 0) (character 0)) (end (line 0) (character 381))))
    (element (id (node (document "d0") (qualified-name "Behaviors::A"))) (kind "kermlDecl") (name "A") (declared-name "A") (range (start (line 1) (character 4)) (end (line 1) (character 125))) (parent (node (document "d0") (qualified-name "Behaviors"))))
    (element (id (node (document "d0") (qualified-name "Behaviors::B"))) (kind "kermlDecl") (name "B") (declared-name "B") (range (start (line 8) (character 4)) (end (line 8) (character 71))) (parent (node (document "d0") (qualified-name "Behaviors"))))
    (element (id (node (document "d0") (qualified-name "Behaviors::C"))) (kind "classifier decl") (name "C") (declared-name "C") (range (start (line 12) (character 4)) (end (line 12) (character 133))) (parent (node (document "d0") (qualified-name "Behaviors"))))
    (element (id (node (document "d0") (qualified-name "Behaviors::msg"))) (kind "flow") (name "msg") (declared-name "msg") (range (start (line 19) (character 4)) (end (line 19) (character 27))) (parent (node (document "d0") (qualified-name "Behaviors"))))
    (element (id (node (document "d0") (qualified-name "Behaviors::msg::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (range (start (line 19) (character 25)) (end (line 19) (character 26))) (parent (node (document "d0") (qualified-name "Behaviors::msg"))) (authored (relationships (typing (reference "C") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Behaviors::msg::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Behaviors::C")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Behaviors::msg::_payload"))) (target (node (document "d0") (qualified-name "Behaviors::C"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Behaviors::msg::_payload"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
