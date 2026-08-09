# META
~~~ini
description=KerML Simple Tests: Inverses
type=file
~~~
# SOURCE
~~~kerml
package Inverses {
	class A {
		feature f : B inverse of B::g disjoint from h;
		feature h : B;
	}
	
	class B {
		feature g : A;
	}
	
	inverse B::g of A::f;
	inverting Invert inverse B::g.f of A::h;
	
	feature gg : A featured by B inverse of A::f;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,KwInverse,KwOf,Ident,ColonColon,Ident,KwDisjoint,KwFrom,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwInverse,Ident,ColonColon,Ident,KwOf,Ident,ColonColon,Ident,Semicolon,
KwInverting,Ident,KwInverse,Ident,ColonColon,Ident,Dot,Ident,KwOf,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,KwFeatured,KwBy,Ident,KwInverse,KwOf,Ident,ColonColon,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Inverses'
    (class_def 'A'
      (feature_def 'f' : 'B' inverse of 'B::g' disjoint from 'h')
      (feature_def 'h' : 'B'))
    (class_def 'B'
      (feature_def 'g' : 'A'))
    (feature_inverting_decl)
    (feature_inverting_decl)
    (feature_def 'gg' : 'A' featured by 'B' inverse of 'A::f')))
~~~
# FORMAT
~~~sysml
package Inverses {
	class A {
		feature f : B inverse of B::g disjoint from h;
		feature h : B;
	}
	
	class B {
		feature g : A;
	}
	
	inverse B::g of A::f;
	inverting Invert inverse B::g.f of A::h;
	
	feature gg : A featured by B inverse of A::f;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Inverses"))) (name "Inverses") (declared-name "Inverses")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Inverses::A"))) (name "A") (declared-name "A"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Inverses::B"))) (name "B") (declared-name "B"))
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
