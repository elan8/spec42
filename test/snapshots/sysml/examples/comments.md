# META
~~~ini
description=SysML Example (Comment): Comments
type=file
~~~
# SOURCE
~~~sysml
package Comments {
	doc /* Documentation Comment */

	doc /* Documentation about Package */

	comment cmt /* Named Comment */	
	comment cmt_cmt about cmt /* Comment about Comment */
	
	comment about C /* Documention Comment on Part Def */
	part def C {
		doc /* Documentation in Part Def */
		comment /* Comment in Part Def */
		comment about Comments /* Comment about Package */
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "comments.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "0e322d9781ce40d4c5e292383b1a32f3fb930c4bf68f3af2c2273f5d341b9054") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Comments"))) (kind "package") (name "Comments") (declared-name "Comments"))
    (element (id (node (document "d0") (qualified-name "Comments::C"))) (kind "part def") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "Comments"))))
    (element (id (node (document "d0") (qualified-name "Comments::C::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Comments::C"))))
    (element (id (node (document "d0") (qualified-name "Comments::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Comments"))))
    (element (id (node (document "d0") (qualified-name "Comments::_documentation#documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Comments"))))
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
