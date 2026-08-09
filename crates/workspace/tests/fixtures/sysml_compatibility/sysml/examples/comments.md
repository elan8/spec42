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
(model
  (namespace
    (package 'Comments'
      (documentation)
      (documentation)
      (comment_annotating 'cmt')
      (comment_annotating 'cmt_cmt')
      (comment_annotating)
      (part_def 'C'
        (documentation)
        (comment_annotating)
        (comment_annotating)))))
~~~
