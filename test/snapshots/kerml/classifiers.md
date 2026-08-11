# META
~~~ini
description=KerML Simple Tests: Classifiers
type=file
~~~
# SOURCE
~~~kerml
package Classifiers {
	classifier A;
	classifier B;
	
	specialization Super subclassifier A specializes B;
	specialization subclassifier B :> A;
	
	subclassifier C specializes A;
	subclassifier C specializes B;
	
	classifier C specializes A, B;
	
	classifier D disjoint from C differences A, B;
	classifier E specializes C intersects A, B;
	classifier F unions A unions B;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "classifiers.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 4 1) (end 4 319))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "bbd89ba924d113a6e36c5064f2984c023c4412681dbd95db31b1488feca8e554") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Classifiers"))) (kind "package") (name "Classifiers") (declared-name "Classifiers"))
    (element (id (node (document "d0") (qualified-name "Classifiers::A"))) (kind "classifier decl") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "Classifiers"))))
    (element (id (node (document "d0") (qualified-name "Classifiers::B"))) (kind "classifier decl") (name "B") (declared-name "B") (parent (node (document "d0") (qualified-name "Classifiers"))))
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
