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
(model
  (namespace
    (package 'Scoping'
      (package 'P1'
        (class_def 'A'
          (feature_def 'f'))
        (package 'P2'
          (class_def 'A'
            (feature_def 'g'))
          (package 'P3'
            (class_def 'B' :> 'Scoping::P1::P2::A'[class_def]
              (feature_def :>> 'Scoping::P1::P2::A::g'[feature_def]))))
        (package 'Objects'
          (class_def 'Object'
            (feature_def 'test1')))
        (package '$'
          (class_def 'Objects'
            (class_def 'Object'
              (feature_def 'test2'))))
        (package 'P4'
          (class_def 'C' :> 'Scoping::P1::Objects::Object'[class_def]
            (feature_def :>> 'Scoping::P1::Objects::Object::test1'[feature_def]))
          (class_def 'D' :> 'Scoping::P1::$::Objects::Object'[class_def]
            (feature_def :>> 'Scoping::P1::$::Objects::Object::test2'[feature_def]))
          (class_def 'E' :> '$::Objects::Object'[unresolved]
            (feature_def :>> 'subobjects'[unresolved])))))))
~~~
