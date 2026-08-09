# META
~~~ini
description=KerML Simple Tests: ArgumentResolution
type=file
~~~
# SOURCE
~~~kerml
package ArgumentResolutionBug {
	class A {
		feature x;
	}
	
	behavior B  {
		in feature x;
		out feature : A = new A(x);
	}
	
	class C {
		feature a : A;
		feature b : B;
		
		connector a ::> a.x to b;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Semicolon,
CloseCurly,
KwBehavior,Ident,OpenCurly,
KwIn,KwFeature,Ident,Semicolon,
KwOut,KwFeature,Colon,Ident,Eq,Ident,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,Semicolon,
KwConnector,Ident,ColonColonGt,Ident,Dot,Ident,KwTo,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ArgumentResolutionBug'
    (class_def 'A'
      (feature_def 'x'))
    (behavior_def
      (feature_def in 'x')
      (feature_def out : 'A' value))
    (class_def 'C'
      (feature_def 'a' : 'A')
      (feature_def 'b' : 'B')
      (connector_def
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package ArgumentResolutionBug {
    class A {
        feature x;
    }

    behavior B {
        in feature x;
        out feature : A = new A(x);
    }

    class C {
        feature a : A;
        feature b : B;

        connector a ::> a.x to b;
    }
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(model
  (namespace
    (package 'ArgumentResolutionBug'
      (class_def 'A'
        (feature_def 'x'))
      (behavior_def 'B'
        (feature_def in 'x')
        (feature_def out : 'ArgumentResolutionBug::A'[class_def]
          (feature_value (=))))
      (class_def 'C'
        (feature_def 'a' : 'ArgumentResolutionBug::A'[class_def])
        (feature_def 'b' : 'ArgumentResolutionBug::B'[behavior_def])
        (connector_def
          (connector_end 'a' :> 'ArgumentResolutionBug::A::x'[feature_def])
          (connector_end 'b'))))))
~~~
