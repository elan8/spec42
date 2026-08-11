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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "inheritance.md"
    (diagnostics
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "11c9649ec23a10bc21a040a438dd1d7f8f7ed187b86045d1a00cc1008019a78e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Inheritance"))) (kind "package") (name "Inheritance") (declared-name "Inheritance") (range (start (line 0) (character 0)) (end (line 0) (character 235))))
    (element (id (node (document "d0") (qualified-name "Inheritance::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 1) (character 1)) (end (line 1) (character 26))) (parent (node (document "d0") (qualified-name "Inheritance"))))
    (element (id (node (document "d0") (qualified-name "Inheritance::B"))) (kind "classifier decl") (name "B") (declared-name "B") (range (start (line 5) (character 1)) (end (line 5) (character 30))) (parent (node (document "d0") (qualified-name "Inheritance"))))
    (element (id (node (document "d0") (qualified-name "Inheritance::us"))) (kind "alias") (name "us") (declared-name "us") (range (start (line 18) (character 1)) (end (line 18) (character 19))) (parent (node (document "d0") (qualified-name "Inheritance"))))
    (element (id (node (document "d0") (qualified-name "Inheritance::w"))) (kind "feature decl") (name "w") (declared-name "w") (range (start (line 16) (character 1)) (end (line 16) (character 21))) (parent (node (document "d0") (qualified-name "Inheritance"))))
    (element (id (node (document "d0") (qualified-name "Inheritance::y"))) (kind "feature decl") (name "y") (declared-name "y") (range (start (line 9) (character 1)) (end (line 9) (character 63))) (parent (node (document "d0") (qualified-name "Inheritance"))))
    (element (id (node (document "d0") (qualified-name "Inheritance::yy"))) (kind "feature decl") (name "yy") (declared-name "yy") (range (start (line 20) (character 1)) (end (line 20) (character 15))) (parent (node (document "d0") (qualified-name "Inheritance"))))
    (element (id (node (document "d0") (qualified-name "Inheritance::z"))) (kind "alias") (name "z") (declared-name "z") (range (start (line 14) (character 1)) (end (line 14) (character 18))) (parent (node (document "d0") (qualified-name "Inheritance"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
