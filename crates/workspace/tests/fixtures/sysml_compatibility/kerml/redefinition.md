# META
~~~ini
description=KerML Simple Tests: Redefinition
type=file
~~~
# SOURCE
~~~kerml
package Redefinition {
	
	classifier A {
	    feature f;
	}
	
	classifier B specializes A {
	    feature redefines f {
	        feature g;
	    }
	}
	
	classifier C specializes A, B {
	    feature subsets f {
	        feature redefines g;
	    }
	}

	class X {
		feature redefines startShot;
		feature redefines endShot;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClassifier,Ident,OpenCurly,
KwFeature,Ident,Semicolon,
CloseCurly,
KwClassifier,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,OpenCurly,
KwFeature,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwClassifier,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwFeature,KwSubsets,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwClass,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Semicolon,
KwFeature,KwRedefines,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Redefinition'
    (classifier_def 'A'
      (feature_def 'f'))
    (classifier_def 'B' :> 'A'
      (feature_def :>> 'f'
        (feature_def 'g')))
    (classifier_def 'C' :> 'A', 'B'
      (feature_def :> 'f'
        (feature_def :>> 'g')))
    (class_def 'X'
      (feature_def :>> 'startShot')
      (feature_def :>> 'endShot'))))
~~~
# FORMAT
~~~sysml
package Redefinition {
	
	classifier A {
	    feature f;
	}
	
	classifier B specializes A {
	    feature redefines f {
	        feature g;
	    }
	}
	
	classifier C specializes A, B {
	    feature subsets f {
	        feature redefines g;
	    }
	}

	class X {
		feature redefines startShot;
		feature redefines endShot;
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'g'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'endShot'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'g'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'endShot'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Redefinition"))) (name "Redefinition") (declared-name "Redefinition")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Redefinition::A"))) (name "A") (declared-name "A"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Redefinition::B"))) (name "B") (declared-name "B"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Redefinition::C"))) (name "C") (declared-name "C"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Redefinition::X"))) (name "X") (declared-name "X"))
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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml/redefinition.md"
    (diagnostics
    )
  )
)
~~~
