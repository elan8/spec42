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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,RegularComment,
KwDoc,RegularComment,
KwComment,Ident,RegularComment,
KwComment,Ident,KwAbout,Ident,RegularComment,
KwComment,KwAbout,Ident,RegularComment,
KwPart,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwComment,RegularComment,
KwComment,KwAbout,Ident,RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Comments'
    (documentation)
    (documentation)
    (comment_annotating 'cmt')
    (comment_annotating 'cmt_cmt' about 'cmt')
    (comment_annotating about 'C')
    (part_def 'C'
      (documentation)
      (comment_annotating)
      (comment_annotating about 'Comments'))))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e3cd3cee75eca71e23e2bfa76db1b3687b2eab7c804dd1bd18e164de8114d0f9") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Comments"))) (kind "package") (name "Comments") (declared-name "Comments") (range (start (line 0) (character 0)) (end (line 0) (character 384))))
    (element (id (node (document "d0") (qualified-name "Comments::C"))) (kind "part def") (name "C") (declared-name "C") (range (start (line 9) (character 1)) (end (line 9) (character 143))) (parent (node (document "d0") (qualified-name "Comments"))))
    (element (id (node (document "d0") (qualified-name "Comments::C::_documentation"))) (kind "documentation") (name "") (range (start (line 9) (character 1)) (end (line 9) (character 143))) (parent (node (document "d0") (qualified-name "Comments::C"))))
    (element (id (node (document "d0") (qualified-name "Comments::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 384))) (parent (node (document "d0") (qualified-name "Comments"))))
    (element (id (node (document "d0") (qualified-name "Comments::_documentation#documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 384))) (parent (node (document "d0") (qualified-name "Comments"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
