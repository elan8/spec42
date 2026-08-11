# META
~~~ini
description=KerML Simple Tests: Associations
type=file
~~~
# SOURCE
~~~kerml
package Associations {
    datatype X;
    class Y;
    
	assoc A {
		end x_cross [1..1] feature x : X; 
		end y_cross [1..*] feature y : Y;
	}
	
	assoc B specializes A {
		end x1;
		end [0..*] feature y1 redefines y;
	}
	
	assoc struct C {
		const end [1] feature a;
		const end feature b;
	}
	
	metaclass M;	
	assoc XY {
		end [0..1] feature x : X {
			@M;
		}
		end feature y : Y;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "associations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Associations {
    datatype X;
    class Y;
    
	assoc A {
		end x_cross [1..1] feature x : X; 
		end y_cross [1..*] feature y : Y;
	}
	
	assoc B specializes A {
		end x1;
		end [0..*] feature y1 redefines y;
	}
	
	assoc struct C {
		const end [1] feature a;
		const end feature b;
	}
	
	metaclass M;	
	assoc XY {
		end [0..1] feature x : X {
			@M;
		}
		end feature y : Y;
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6dbd1ee2ee4067baf63619d64fb6aadcd23df9def32b99f65ab68ebc4b86b1fe") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Associations"))) (kind "package") (name "Associations") (declared-name "Associations") (range (start (line 0) (character 0)) (end (line 0) (character 388))))
    (element (id (node (document "d0") (qualified-name "Associations::A"))) (kind "kermlDecl") (name "A") (declared-name "A") (range (start (line 4) (character 1)) (end (line 4) (character 86))) (parent (node (document "d0") (qualified-name "Associations"))))
    (element (id (node (document "d0") (qualified-name "Associations::B"))) (kind "kermlDecl") (name "B") (declared-name "B") (range (start (line 9) (character 1)) (end (line 9) (character 74))) (parent (node (document "d0") (qualified-name "Associations"))))
    (element (id (node (document "d0") (qualified-name "Associations::M"))) (kind "kermlDecl") (name "M") (declared-name "M") (range (start (line 19) (character 1)) (end (line 19) (character 13))) (parent (node (document "d0") (qualified-name "Associations"))))
    (element (id (node (document "d0") (qualified-name "Associations::X"))) (kind "kermlDecl") (name "X") (declared-name "X") (range (start (line 1) (character 4)) (end (line 1) (character 15))) (parent (node (document "d0") (qualified-name "Associations"))))
    (element (id (node (document "d0") (qualified-name "Associations::XY"))) (kind "kermlDecl") (name "XY") (declared-name "XY") (range (start (line 20) (character 1)) (end (line 20) (character 75))) (parent (node (document "d0") (qualified-name "Associations"))))
    (element (id (node (document "d0") (qualified-name "Associations::Y"))) (kind "classifier decl") (name "Y") (declared-name "Y") (range (start (line 2) (character 4)) (end (line 2) (character 12))) (parent (node (document "d0") (qualified-name "Associations"))))
    (element (id (node (document "d0") (qualified-name "Associations::struct"))) (kind "kermlDecl") (name "struct") (declared-name "struct") (range (start (line 14) (character 1)) (end (line 14) (character 70))) (parent (node (document "d0") (qualified-name "Associations"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
