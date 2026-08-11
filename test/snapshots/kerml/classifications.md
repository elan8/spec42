# META
~~~ini
description=KerML Simple Tests: Classifications
type=file
~~~
# SOURCE
~~~kerml
package Classifications {
	class T;
	x;
	y = x istype T or x hastype z;
	z = (all T)#(3);
	a = x as T;
	b = x meta KerML::Feature;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "classifications.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 2 1) (end 2 95))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "a00cc4b54d97a73467e6dc373e30d09d4ba1cbe574305005b3848236ff9fea71") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Classifications"))) (kind "package") (name "Classifications") (declared-name "Classifications"))
    (element (id (node (document "d0") (qualified-name "Classifications::T"))) (kind "classifier decl") (name "T") (declared-name "T") (parent (node (document "d0") (qualified-name "Classifications"))))
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
