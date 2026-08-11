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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "conjugation.md"
    (diagnostics
    )
  )
)
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
# EXPECTED
~~~
semantic.unresolved_name 'B::f'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'B::f'
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "22cf655c28c2ced4bcf0e4f4b5e79635beed6c52bea2a080ed7805b7f134dc7c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Conjugation"))) (kind "package") (name "Conjugation") (declared-name "Conjugation") (range (start (line 0) (character 0)) (end (line 0) (character 99))))
    (element (id (node (document "d0") (qualified-name "Conjugation::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 1) (character 1)) (end (line 1) (character 29))) (parent (node (document "d0") (qualified-name "Conjugation"))))
    (element (id (node (document "d0") (qualified-name "Conjugation::B"))) (kind "classifier decl") (name "B") (declared-name "B") (range (start (line 5) (character 1)) (end (line 5) (character 22))) (parent (node (document "d0") (qualified-name "Conjugation"))))
    (element (id (node (document "d0") (qualified-name "Conjugation::g"))) (kind "feature decl") (name "g") (declared-name "g") (range (start (line 7) (character 1)) (end (line 7) (character 18))) (parent (node (document "d0") (qualified-name "Conjugation"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
