# META
~~~ini
description=KerML Simple Tests: Classifications
type=file
~~~
# SOURCE
~~~kerml
package Classifications {
	class T;
	x;
	y = x istype T or x hastype z;
	z = (all T)#(3);
	a = x as T;
	b = x meta KerML::Feature;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,Semicolon,
Ident,Semicolon,
Ident,Eq,Ident,KwIstype,Ident,KwOr,Ident,KwHastype,Ident,Semicolon,
Ident,Eq,OpenParen,KwAll,Ident,CloseParen,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
Ident,Eq,Ident,KwAs,Ident,Semicolon,
Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Classifications'
    (class_def 'T')
    (feature_def 'x')
    (feature_def 'y' value)
    (feature_def 'z' value)
    (feature_def 'a' value)
    (feature_def 'b' value)))
~~~
# FORMAT
~~~sysml
package Classifications {
    class T;
    x;
    y = x istype T or x hastype z;
    z = (all T)#(3);
    a = x as T;
    b = x meta KerML::Feature;
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
    (package 'Classifications'
      (class_def 'T')
      (feature_def 'x')
      (feature_def 'y'
        (feature_value (=)))
      (feature_def 'z'
        (feature_value (=)))
      (feature_def 'a'
        (feature_value (=)))
      (feature_def 'b'
        (feature_value (=))))))
~~~
