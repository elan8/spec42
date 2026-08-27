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
  (document "memory://snapshot/comments.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e183ded80174e3b0ddd4c289f66fa542e972a5051a9a34c42fc99cfc114f4941"))
  (declarations
    (declaration (id (node (document "memory://snapshot/comments.md") (qualified-name "Comments"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text " Documentation Comment ")) (doc (text " Documentation about Package ")) (comment (text " Named Comment ")) (comment (text " Comment about Comment ")) (comment (text " Documention Comment on Part Def "))))
    (declaration (id (node (document "memory://snapshot/comments.md") (qualified-name "Comments::C"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Documentation in Part Def ")) (comment (text " Comment in Part Def ")) (comment (text " Comment about Package "))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
