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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Scoping"))) (name "Scoping") (declared-name "Scoping")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "Scoping::P1"))) (name "P1") (declared-name "P1")
          (contains
            (element (kind "package") (id (node (document "d0") (qualified-name "Scoping::P1::$"))) (name "$") (declared-name "$")
              (contains
                (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Scoping::P1::$::Objects"))) (name "Objects") (declared-name "Objects"))
              )
            )
            (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Scoping::P1::A"))) (name "A") (declared-name "A"))
            (element (kind "package") (id (node (document "d0") (qualified-name "Scoping::P1::Objects"))) (name "Objects") (declared-name "Objects")
              (contains
                (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Scoping::P1::Objects::Object"))) (name "Object") (declared-name "Object"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "Scoping::P1::P2"))) (name "P2") (declared-name "P2")
              (contains
                (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Scoping::P1::P2::A"))) (name "A") (declared-name "A"))
                (element (kind "package") (id (node (document "d0") (qualified-name "Scoping::P1::P2::P3"))) (name "P3") (declared-name "P3")
                  (contains
                    (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Scoping::P1::P2::P3::B"))) (name "B") (declared-name "B"))
                  )
                )
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "Scoping::P1::P4"))) (name "P4") (declared-name "P4")
              (contains
                (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Scoping::P1::P4::C"))) (name "C") (declared-name "C"))
                (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Scoping::P1::P4::D"))) (name "D") (declared-name "D"))
                (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Scoping::P1::P4::E"))) (name "E") (declared-name "E"))
              )
            )
          )
        )
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
