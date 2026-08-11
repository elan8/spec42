# META
~~~ini
description=KerML Simple Tests: Scoping
type=file
~~~
# SOURCE
~~~kerml
package Scoping {
    package P1 {
        class A {
            feature f;
        }
        package P2 {
            class A {
                feature g;
            }
            package P3 {
                class B :> A {
                    feature :>> g;
                }
            }
        }
        package Objects {
            class Object {
                feature test1;
            }
        }
        package '$' {
            class Objects {
                class Object {
                    feature test2;
                }
            }
        }
        package P4 {
            class C :> Objects::Object {
                feature :>> test1;
            }
            class D :> '$'::Objects::Object {
                feature :>> test2;
            }
            class E :> $::Objects::Object {
                feature :>> subobjects;
            }
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "scoping.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d9c54ac853a06f36f18e0cb28deaae70a122cd1a5c3739092b1eda62cfc99d69") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Scoping"))) (kind "package") (name "Scoping") (declared-name "Scoping"))
    (element (id (node (document "d0") (qualified-name "Scoping::P1"))) (kind "package") (name "P1") (declared-name "P1") (parent (node (document "d0") (qualified-name "Scoping"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::$"))) (kind "package") (name "$") (declared-name "$") (parent (node (document "d0") (qualified-name "Scoping::P1"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::$::Objects"))) (kind "classifier decl") (name "Objects") (declared-name "Objects") (parent (node (document "d0") (qualified-name "Scoping::P1::$"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::A"))) (kind "classifier decl") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "Scoping::P1"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::Objects"))) (kind "package") (name "Objects") (declared-name "Objects") (parent (node (document "d0") (qualified-name "Scoping::P1"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::Objects::Object"))) (kind "classifier decl") (name "Object") (declared-name "Object") (parent (node (document "d0") (qualified-name "Scoping::P1::Objects"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P2"))) (kind "package") (name "P2") (declared-name "P2") (parent (node (document "d0") (qualified-name "Scoping::P1"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P2::A"))) (kind "classifier decl") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "Scoping::P1::P2"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P2::P3"))) (kind "package") (name "P3") (declared-name "P3") (parent (node (document "d0") (qualified-name "Scoping::P1::P2"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P2::P3::B"))) (kind "classifier decl") (name "B") (declared-name "B") (parent (node (document "d0") (qualified-name "Scoping::P1::P2::P3"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P4"))) (kind "package") (name "P4") (declared-name "P4") (parent (node (document "d0") (qualified-name "Scoping::P1"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P4::C"))) (kind "classifier decl") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "Scoping::P1::P4"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P4::D"))) (kind "classifier decl") (name "D") (declared-name "D") (parent (node (document "d0") (qualified-name "Scoping::P1::P4"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P4::E"))) (kind "classifier decl") (name "E") (declared-name "E") (parent (node (document "d0") (qualified-name "Scoping::P1::P4"))))
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
