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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "argument_resolution.md"
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "168d41ced08f9a9163ae5446c034a66acc1a27d514697af3ccb6cfdf8df87f88") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ArgumentResolutionBug"))) (kind "package") (name "ArgumentResolutionBug") (declared-name "ArgumentResolutionBug") (range (start (line 0) (character 0)) (end (line 0) (character 207))))
    (element (id (node (document "d0") (qualified-name "ArgumentResolutionBug::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 1) (character 1)) (end (line 1) (character 26))) (parent (node (document "d0") (qualified-name "ArgumentResolutionBug"))))
    (element (id (node (document "d0") (qualified-name "ArgumentResolutionBug::B"))) (kind "kermlDecl") (name "B") (declared-name "B") (range (start (line 5) (character 1)) (end (line 5) (character 63))) (parent (node (document "d0") (qualified-name "ArgumentResolutionBug"))))
    (element (id (node (document "d0") (qualified-name "ArgumentResolutionBug::C"))) (kind "classifier decl") (name "C") (declared-name "C") (range (start (line 10) (character 1)) (end (line 10) (character 78))) (parent (node (document "d0") (qualified-name "ArgumentResolutionBug"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
