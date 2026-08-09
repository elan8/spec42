# META
~~~ini
description=KerML Simple Tests: Inheritance
type=file
~~~
# SOURCE
~~~kerml
package Inheritance {
	class A {
		feature f;
	}
	
	class B specializes A {
		
	}
		
	feature y: A {
		alias x for B::f;
		feature g redefines f;
	}
	
	alias z for y::g;
	
	feature w subsets y;
	
	alias us for w::g;
	
	feature yy: y;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Semicolon,
CloseCurly,
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenCurly,
KwAlias,Ident,KwFor,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAlias,Ident,KwFor,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,KwSubsets,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Inheritance'
    (class_def 'A'
      (feature_def 'f'))
    (class_def 'B' :> 'A')
    (feature_def 'y' : 'A'
      (alias_member 'x' for 'B::f')
      (feature_def 'g' :>> 'f'))
    (alias_member 'z' for 'y::g')
    (feature_def 'w' :> 'y')
    (alias_member 'us' for 'w::g')
    (feature_def 'yy' : 'y')))
~~~
# FORMAT
~~~sysml
package Inheritance {
    class A {
        feature f;
    }

    class B specializes A { }

    feature y : A {
        alias x for B::f;
        feature g redefines f;
    }

    alias z for y::g;

    feature w subsets y;

    alias us for w::g;

    feature yy : y;
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
    (package 'Inheritance'
      (class_def 'A'
        (feature_def 'f'))
      (class_def 'B' :> 'Inheritance::A'[class_def])
      (feature_def 'y' : 'Inheritance::A'[class_def]
        (alias_member 'x' -> 'Inheritance::A::f'[feature_def])
        (feature_def 'g' :>> 'Inheritance::A::f'[feature_def]))
      (alias_member 'z' -> 'Inheritance::y::g'[feature_def])
      (feature_def 'w' :> 'Inheritance::y'[feature_def])
      (alias_member 'us' -> 'Inheritance::y::g'[feature_def])
      (feature_def 'yy' : 'Inheritance::y'[feature_def]))))
~~~
