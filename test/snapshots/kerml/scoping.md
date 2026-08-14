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
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 3 12) (end 4 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 7 16) (end 8 12))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 11 20) (end 12 16))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 17 16) (end 18 12))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 22 16) (end 25 12))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 29 16) (end 30 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 31 23) (end 31 43))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 32 16) (end 33 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 34 23) (end 34 41))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 35 16) (end 36 12))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:dc02369d6fa0296eaa75f6ba3e3e3047242d8a9310da6da02398d3e536f61e83") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::$::Objects"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3::B"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A")))))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::C"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Objects::Object")))))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::D"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "$::Objects::Object")))))
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::E"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "$::Objects::Object")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3::B"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A")))))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::C"))) (kind specialization) (ordinal 0))
      (authored-target "Objects::Object")
      (outcome (status resolved) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object")))))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::D"))) (kind specialization) (ordinal 0))
      (authored-target "$::Objects::Object")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::E"))) (kind specialization) (ordinal 0))
      (authored-target "$::Objects::Object")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3::B"))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3::B"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::C"))) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::C"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::P3::B")))
      (supertype (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P2::A")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::C")))
      (supertype (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object")) (scopes any subclassification))
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
  (query (document "memory://snapshot/scoping.md") (range (start 28 23) (end 28 38)) (probe (position 28 23))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::C"))) (kind specialization) (ordinal 0) (authored-target "Objects::Object")
      (outcome (status resolved) (target (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::Objects::Object")))))
    )
  )
  (query (document "memory://snapshot/scoping.md") (range (start 31 23) (end 31 43)) (probe (position 31 23))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::D"))) (kind specialization) (ordinal 0) (authored-target "$::Objects::Object")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/scoping.md") (range (start 34 23) (end 34 41)) (probe (position 34 23))
    (reference (id (source (node (document "memory://snapshot/scoping.md") (qualified-name "Scoping::P1::P4::E"))) (kind specialization) (ordinal 0) (authored-target "$::Objects::Object")
      (outcome (status unresolved)))
    )
  )
)
~~~
