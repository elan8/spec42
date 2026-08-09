# META
~~~ini
description=KerML Simple Tests: Circular
type=file
~~~
# SOURCE
~~~kerml
package Circular {
	class A { }
	feature a: A;
	alias Circ for Circular;
	package P {
		public import Circular::*;
	}
	
	feature x :> z;
	feature y :> x;
	feature z :> y;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,CloseCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
CloseCurly,
KwFeature,Ident,ColonGt,Ident,Semicolon,
KwFeature,Ident,ColonGt,Ident,Semicolon,
KwFeature,Ident,ColonGt,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Circular'
    (class_def 'A')
    (feature_def 'a' : 'A')
    (alias_member 'Circ' for 'Circular')
    (package_def 'P'
      (import_decl public 'Circular::*'))
    (feature_def 'x' :> 'z')
    (feature_def 'y' :> 'x')
    (feature_def 'z' :> 'y')))
~~~
# FORMAT
~~~sysml
package Circular {
    class A { }
    feature a : A;
    alias Circ for Circular;
    package P {
        public import Circular::*;
    }

    feature x :> z;
    feature y :> x;
    feature z :> y;
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
    (package 'Circular'
      (class_def 'A')
      (feature_def 'a' : 'Circular::A'[class_def])
      (alias_member 'Circ' -> 'Circular'[package])
      (package 'P'
        (namespace_import public -> 'Circular'[package]))
      (feature_def 'x' :> 'Circular::z'[feature_def])
      (feature_def 'y' :> 'Circular::x'[feature_def])
      (feature_def 'z' :> 'Circular::y'[feature_def]))))
~~~
