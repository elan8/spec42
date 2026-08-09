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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Inheritance"))) (name "Inheritance") (declared-name "Inheritance")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Inheritance::A"))) (name "A") (declared-name "A"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Inheritance::B"))) (name "B") (declared-name "B"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "Inheritance::us"))) (name "us") (declared-name "us"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Inheritance::w"))) (name "w") (declared-name "w"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Inheritance::y"))) (name "y") (declared-name "y"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Inheritance::yy"))) (name "yy") (declared-name "yy"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "Inheritance::z"))) (name "z") (declared-name "z"))
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
