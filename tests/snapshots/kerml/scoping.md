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
  (document "memory://snapshot/scoping.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 34 23) (end 34 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 35 28) (end 35 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:dc02369d6fa0296eaa75f6ba3e3e3047242d8a9310da6da02398d3e536f61e83"))
  (declarations
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects::Object"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects::Object::test2"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::A::f"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object::test1"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A::g"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3::B"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A")))))
    (declaration (id (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P2")) (named (kind package) (name "P3")) (named (kind class-def) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (effective-identification (name "g") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "g")))))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::C"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Objects::Object")))))
    (declaration (id (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "C")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (effective-identification (name "test1") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "test1")))))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::D"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "$::Objects::Object")))))
    (declaration (id (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "D")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (effective-identification (name "test2") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "test2")))))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::E"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "$::Objects::Object")))))
    (declaration (id (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "E")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subobjects")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3::B"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A")))))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P2")) (named (kind package) (name "P3")) (named (kind class-def) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "g")
      (outcome (status resolved) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A::g")))))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::C"))) (kind specialization) (ordinal 0))
      (authored-target "Objects::Object")
      (outcome (status resolved) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object")))))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "C")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "test1")
      (outcome (status resolved) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object::test1")))))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::D"))) (kind specialization) (ordinal 0))
      (authored-target "$::Objects::Object")
      (outcome (status resolved) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects::Object")))))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "D")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "test2")
      (outcome (status resolved) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects::Object::test2")))))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::E"))) (kind specialization) (ordinal 0))
      (authored-target "$::Objects::Object")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "E")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "subobjects")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3::B"))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3::B"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P2")) (named (kind package) (name "P3")) (named (kind class-def) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A::g"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P2")) (named (kind package) (name "P3")) (named (kind class-def) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::C"))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::C"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "C")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object::test1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "C")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::D"))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects::Object"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::D"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "D")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects::Object::test2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "D")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects::Object::test2"))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects::Object"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::A::f"))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object::test1"))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A::g"))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P2")) (named (kind package) (name "P3")) (named (kind class-def) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3::B"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "C")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "D")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::D"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "E")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::E"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects::Object")))
      (subtype (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::D")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects::Object::test2")))
      (featured-by (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects::Object")))
      (subtype (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "D")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::A::f")))
      (featured-by (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::A")))
    )
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object")))
      (subtype (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::C")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object::test1")))
      (featured-by (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object")))
      (subtype (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "C")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A")))
      (subtype (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3::B")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A::g")))
      (featured-by (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A")))
      (subtype (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P2")) (named (kind package) (name "P3")) (named (kind class-def) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3::B")))
      (supertype (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P2")) (named (kind package) (name "P3")) (named (kind class-def) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3::B")))
      (supertype (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A::g")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::C")))
      (supertype (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "C")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::C")))
      (supertype (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object::test1")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::D")))
      (supertype (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects::Object")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "D")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::D")))
      (supertype (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects::Object::test2")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "E")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::E")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/scoping.md") (range (start 10 27) (end 10 28)) (probe (position 10 27))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3::B"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A")))))
    )
  )
  (query (document "memory://snapshot/scoping.md") (range (start 11 32) (end 11 33)) (probe (position 11 32))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P2")) (named (kind package) (name "P3")) (named (kind class-def) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "g")
      (outcome (status resolved) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A::g")))))
    )
  )
  (query (document "memory://snapshot/scoping.md") (range (start 28 23) (end 28 38)) (probe (position 28 23))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::C"))) (kind specialization) (ordinal 0) (authored-target "Objects::Object")
      (outcome (status resolved) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object")))))
    )
  )
  (query (document "memory://snapshot/scoping.md") (range (start 29 28) (end 29 33)) (probe (position 29 28))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "C")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "test1")
      (outcome (status resolved) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object::test1")))))
    )
  )
  (query (document "memory://snapshot/scoping.md") (range (start 31 23) (end 31 43)) (probe (position 31 23))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::D"))) (kind specialization) (ordinal 0) (authored-target "$::Objects::Object")
      (outcome (status resolved) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects::Object")))))
    )
  )
  (query (document "memory://snapshot/scoping.md") (range (start 32 28) (end 32 33)) (probe (position 32 28))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "D")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "test2")
      (outcome (status resolved) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects::Object::test2")))))
    )
  )
  (query (document "memory://snapshot/scoping.md") (range (start 34 23) (end 34 41)) (probe (position 34 23))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::E"))) (kind specialization) (ordinal 0) (authored-target "$::Objects::Object")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/scoping.md") (range (start 35 28) (end 35 38)) (probe (position 35 28))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (path (named (kind package) (name "Scoping")) (named (kind package) (name "P1")) (named (kind package) (name "P4")) (named (kind class-def) (name "E")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "subobjects")
      (outcome (status unresolved)))
    )
  )
)
~~~
