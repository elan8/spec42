# META
~~~ini
description=KerML Simple Tests: Conjugation
type=file
~~~
# SOURCE
~~~kerml
package Conjugation {
	class A {
		in feature f;
	}
	
	class B conjugates A;
	
	feature g ~ B::f;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,
KwIn,KwFeature,Ident,Semicolon,
CloseCurly,
KwClass,Ident,KwConjugates,Ident,Semicolon,
KwFeature,Ident,Tilde,Ident,ColonColon,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Conjugation'
    (class_def 'A'
      (feature_def in 'f'))
    (class_def 'B' ~ 'A')
    (feature_def 'g' ~ B::f)))
~~~
# FORMAT
~~~sysml
package Conjugation {
    class A {
        in feature f;
    }

    class B conjugates A;

    feature g ~ B::f;
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'B::f'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'B::f'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Conjugation'
      (class_def 'A'
        (feature_def in 'f'))
      (class_def 'B' ~ 'Conjugation::A'[class_def])
      (feature_def 'g' ~ 'B::f'[unresolved]))))
~~~
