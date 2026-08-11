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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwClass,Ident,ColonGt,Ident,OpenCurly,
KwFeature,ColonGtGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
KwClass,Ident,OpenCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwClass,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwFeature,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwClass,Ident,ColonGt,UnrestrictedName,ColonColon,Ident,ColonColon,Ident,OpenCurly,
KwFeature,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwClass,Ident,ColonGt,Dollar,ColonColon,Ident,ColonColon,Ident,OpenCurly,
KwFeature,ColonGtGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Scoping'
    (package_def 'P1'
      (class_def 'A'
        (feature_def 'f'))
      (package_def 'P2'
        (class_def 'A'
          (feature_def 'g'))
        (package_def 'P3'
          (class_def 'B' :> 'A'
            (feature_def :>> 'g'))))
      (package_def 'Objects'
        (class_def 'Object'
          (feature_def 'test1')))
      (package_def ''$''
        (class_def 'Objects'
          (class_def 'Object'
            (feature_def 'test2'))))
      (package_def 'P4'
        (class_def 'C' :> 'Objects::Object'
          (feature_def :>> 'test1'))
        (class_def 'D' :> ''$'::Objects::Object'
          (feature_def :>> 'test2'))
        (class_def 'E' :> '$::Objects::Object'
          (feature_def :>> 'subobjects'))))))
~~~
# EXPECTED
~~~
semantic.unresolved_name '$::Objects::Object'
semantic.unresolved_name 'subobjects'
~~~
# PROBLEMS
~~~
semantic.unresolved_name '$::Objects::Object'
semantic.unresolved_name 'subobjects'
~~~
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d9c54ac853a06f36f18e0cb28deaae70a122cd1a5c3739092b1eda62cfc99d69") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Scoping"))) (kind "package") (name "Scoping") (declared-name "Scoping") (range (start (line 0) (character 0)) (end (line 0) (character 890))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1"))) (kind "package") (name "P1") (declared-name "P1") (range (start (line 1) (character 4)) (end (line 1) (character 870))) (parent (node (document "d0") (qualified-name "Scoping"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::$"))) (kind "package") (name "$") (declared-name "$") (range (start (line 20) (character 8)) (end (line 20) (character 157))) (parent (node (document "d0") (qualified-name "Scoping::P1"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::$::Objects"))) (kind "classifier decl") (name "Objects") (declared-name "Objects") (range (start (line 21) (character 12)) (end (line 21) (character 125))) (parent (node (document "d0") (qualified-name "Scoping::P1::$"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 2) (character 8)) (end (line 2) (character 50))) (parent (node (document "d0") (qualified-name "Scoping::P1"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::Objects"))) (kind "package") (name "Objects") (declared-name "Objects") (range (start (line 15) (character 8)) (end (line 15) (character 107))) (parent (node (document "d0") (qualified-name "Scoping::P1"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::Objects::Object"))) (kind "classifier decl") (name "Object") (declared-name "Object") (range (start (line 16) (character 12)) (end (line 16) (character 71))) (parent (node (document "d0") (qualified-name "Scoping::P1::Objects"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P2"))) (kind "package") (name "P2") (declared-name "P2") (range (start (line 5) (character 8)) (end (line 5) (character 216))) (parent (node (document "d0") (qualified-name "Scoping::P1"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P2::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 6) (character 12)) (end (line 6) (character 62))) (parent (node (document "d0") (qualified-name "Scoping::P1::P2"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P2::P3"))) (kind "package") (name "P3") (declared-name "P3") (range (start (line 9) (character 12)) (end (line 9) (character 122))) (parent (node (document "d0") (qualified-name "Scoping::P1::P2"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P2::P3::B"))) (kind "classifier decl") (name "B") (declared-name "B") (range (start (line 10) (character 16)) (end (line 10) (character 83))) (parent (node (document "d0") (qualified-name "Scoping::P1::P2::P3"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P4"))) (kind "package") (name "P4") (declared-name "P4") (range (start (line 27) (character 8)) (end (line 27) (character 313))) (parent (node (document "d0") (qualified-name "Scoping::P1"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P4::C"))) (kind "classifier decl") (name "C") (declared-name "C") (range (start (line 28) (character 12)) (end (line 28) (character 89))) (parent (node (document "d0") (qualified-name "Scoping::P1::P4"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P4::D"))) (kind "classifier decl") (name "D") (declared-name "D") (range (start (line 31) (character 12)) (end (line 31) (character 94))) (parent (node (document "d0") (qualified-name "Scoping::P1::P4"))))
    (element (id (node (document "d0") (qualified-name "Scoping::P1::P4::E"))) (kind "classifier decl") (name "E") (declared-name "E") (range (start (line 34) (character 12)) (end (line 34) (character 97))) (parent (node (document "d0") (qualified-name "Scoping::P1::P4"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
