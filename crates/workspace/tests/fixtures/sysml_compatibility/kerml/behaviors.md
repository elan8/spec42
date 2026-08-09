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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwBehavior,Ident,OpenCurly,
KwIn,Ident,Semicolon,
KwOut,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwComposite,KwStep,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwIn,Ident,Semicolon,
KwOut,KwVar,Ident,Semicolon,
CloseCurly,
KwClass,Ident,OpenCurly,
KwVar,Ident,Eq,Ident,OpenParen,CloseParen,Dot,Ident,Semicolon,
KwStep,Ident,Colon,Ident,Semicolon,
KwStep,Ident,Colon,Ident,Semicolon,
KwBinding,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAbstract,KwFlow,Ident,KwOf,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Behaviors'
    (behavior_def
      (feature_def in 'x')
      (feature_def out 'y' value)
      (step_def
        (feature_def in 'x1' value)))
    (behavior_def
      (feature_def in 'x1')
      (feature_def out var 'y1'))
    (class_def 'C'
      (feature_def var 'z' value)
      (step_def)
      (step_def)
      (binding_connector
        (connector_end)
        (connector_end))
      (flow_feature 'a'))
    (flow_usage 'msg' : 'C')))
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
        flow a;
    }
    abstract flow msg of C;
}
~~~
# EXPECTED
~~~
semantic.duplicate_name 'a'
semantic.ambiguous_member 'a'
semantic.invalid_connection_end_count
semantic.feature_typing_kind_mismatch
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'a'
semantic.ambiguous_member 'a'
semantic.invalid_connection_end_count
semantic.feature_typing_kind_mismatch
~~~
# SMG
~~~
(model
  (namespace
    (package 'Behaviors'
      (behavior_def 'A'
        (feature_def in 'x')
        (feature_def out 'y'
          (feature_value (=)))
        (step_def composite 'b' : 'Behaviors::B'[behavior_def]
          (feature_def in 'x1'
            (feature_value (=)))))
      (behavior_def 'B' :> 'Behaviors::A'[behavior_def]
        (feature_def in 'x1' :>> 'Behaviors::A::x'[feature_def][implied])
        (feature_def out 'y1' :>> 'Behaviors::A::y'[feature_def][implied]))
      (class_def 'C'
        (feature_def 'z'
          (feature_value (=)))
        (step_def 'a' : 'Behaviors::A'[behavior_def])
        (step_def 'b' : 'Behaviors::B'[behavior_def])
        (binding_connector_def
          (connector_end 'z')
          (connector_end 'a.y'))
        (flow_usage composite 'a'))
      (flow_usage abstract 'msg' : 'Behaviors::C'[class_def]))))
~~~
