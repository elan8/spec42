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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "inverses.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 10 1) (end 10 114))
      )
    )
  )
)
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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "4ae52aa499248f0bbc9aa17266e6a7dd3df26dde1807060e2d796bcc9567831b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Inverses"))) (kind "package") (name "Inverses") (declared-name "Inverses") (range (start (line 0) (character 0)) (end (line 0) (character 249))))
    (element (id (node (document "d0") (qualified-name "Inverses::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 1) (character 1)) (end (line 1) (character 79))) (parent (node (document "d0") (qualified-name "Inverses"))))
    (element (id (node (document "d0") (qualified-name "Inverses::B"))) (kind "classifier decl") (name "B") (declared-name "B") (range (start (line 6) (character 1)) (end (line 6) (character 30))) (parent (node (document "d0") (qualified-name "Inverses"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
