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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "redefinition.md"
    (diagnostics
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a65431c61d3eae2ccdaa78118065520d6e574757c8aa3f545e4464519399fcee") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Redefinition"))) (kind "package") (name "Redefinition") (declared-name "Redefinition") (range (start (line 0) (character 0)) (end (line 0) (character 325))))
    (element (id (node (document "d0") (qualified-name "Redefinition::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 2) (character 1)) (end (line 2) (character 34))) (parent (node (document "d0") (qualified-name "Redefinition"))))
    (element (id (node (document "d0") (qualified-name "Redefinition::B"))) (kind "classifier decl") (name "B") (declared-name "B") (range (start (line 6) (character 1)) (end (line 6) (character 86))) (parent (node (document "d0") (qualified-name "Redefinition"))))
    (element (id (node (document "d0") (qualified-name "Redefinition::C"))) (kind "classifier decl") (name "C") (declared-name "C") (range (start (line 12) (character 1)) (end (line 12) (character 97))) (parent (node (document "d0") (qualified-name "Redefinition"))))
    (element (id (node (document "d0") (qualified-name "Redefinition::X"))) (kind "classifier decl") (name "X") (declared-name "X") (range (start (line 18) (character 1)) (end (line 18) (character 73))) (parent (node (document "d0") (qualified-name "Redefinition"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
