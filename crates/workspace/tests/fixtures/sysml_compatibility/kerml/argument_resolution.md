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
    (element (kind "package") (id (node (document "d0") (qualified-name "ArgumentResolutionBug"))) (name "ArgumentResolutionBug") (declared-name "ArgumentResolutionBug")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ArgumentResolutionBug::A"))) (name "A") (declared-name "A"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ArgumentResolutionBug::B"))) (name "B") (declared-name "B"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ArgumentResolutionBug::C"))) (name "C") (declared-name "C"))
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
