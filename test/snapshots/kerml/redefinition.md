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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "89d27b100c3658ab2d7e343244ec744434896ed3c94f38880f6a712275280140") (contract-version "canonical-resolution-v1"))
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
# NAVIGATION
~~~sexpr
(navigation
)
~~~
